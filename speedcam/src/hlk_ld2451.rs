use std::{time::Duration};

use anyhow::{Context, Result, anyhow};

// Assumption: sensor frames are delimited with a 2-byte header and 2-byte footer.
// These values can be changed to match the actual device protocol.
const HEADER: &[u8] = &[0xF4, 0xF3, 0xF2, 0xF1];
const FOOTER: &[u8] = &[0xF8, 0xF7, 0xF6, 0xF5];

pub struct Radar {
    port: Box<dyn serialport::SerialPort>,
    // internal buffer to accumulate stream bytes across calls
    buffer: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Approaching,
    Receding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TargetData {
    pub angle: i8,
    pub distance: u8,
    pub direction: Direction,
    pub speed: u8,
    pub snr: u8,
}

impl Radar {
    pub fn new(port_name: &str) -> Result<Self> {
        let port = serialport::new(port_name, 115_200)
            .timeout(Duration::from_millis(10))
            .open()
            .context("Failed to open HLK-LD2451 serial port")?;

        log::debug!("Successfully connected to HLK-LD2451 on port {}", port_name);

        Ok(Radar { port, buffer: Vec::new() })
    }

    pub fn flush(&mut self) -> Result<()> {
        self.port.clear(serialport::ClearBuffer::Input).context("Failed to flush HLK-LD2451 input buffer")?;
        self.buffer.clear();
        Ok(())
    }

    /// Read data from the radar. This method is stateful: it accumulates bytes in an
    /// internal buffer until it finds the header, then waits for a footer and parses
    /// the payload between header and footer as a sequence of 8-byte target records
    /// (4 bytes little-endian float distance, 4 bytes little-endian float speed).
    pub fn read_targets(&mut self) -> Result<Vec<TargetData>> {
        // Read any available bytes and append to internal buffer
        let available = self.port.bytes_to_read()?;
        if available > 0 {
            let mut tmp = vec![0u8; available as usize];
            let bytes_read = self.port.read(&mut tmp).context("Failed to read from HLK-LD2451")?;
            tmp.truncate(bytes_read);
            self.buffer.extend_from_slice(&tmp);
        } else {
            return Ok(Vec::new());
        }

        // Search for header in the buffer
        if let Some(header_start) = self.buffer.windows(HEADER.len()).position(|w| w == HEADER) {
            // Found header, now look for two byte frame length.
            if self.buffer.len() < header_start + HEADER.len() + 2 {
                // Not enough data yet to read frame length.
                return Ok(Vec::new());
            }

            let frame_length = u16::from_le_bytes([
                self.buffer[header_start + HEADER.len()],
                self.buffer[header_start + HEADER.len() + 1],
            ]) as usize;

            let frame_start = header_start + HEADER.len() + 2;
            let frame_end = frame_start + frame_length;
            let footer_start = frame_end;
            let footer_end = footer_start + FOOTER.len();
            
            // Now check if we have the full frame (header + length + payload + footer)
            if self.buffer.len() < footer_end {
                // Not enough data yet to read full frame.
                return Ok(Vec::new());
            }

            // Check footer
            if &self.buffer[footer_start..footer_end] != FOOTER {
                self.buffer.drain(0..footer_end);
                log::warn!("Invalid HLK-LD2451 footer, discarding data up to end of footer");
                return Ok(Vec::new());
            }

            if frame_length >= 2 {
                let targets = match self.parse_frame(&self.buffer[frame_start..frame_end]) {
                    Ok(t) => Ok(t),
                    Err(e) => {
                        log::warn!("Failed to parse HLK-LD2451 frame: {}", e);
                        // Discard this invalid frame
                        self.buffer.drain(0..footer_end);
                        return Ok(Vec::new());
                    }
                };
                // Remove processed frame from buffer
                self.buffer.drain(0..footer_end);
                targets
            } else {
                // Discard empty frame
                self.buffer.drain(0..footer_end);
                Ok(Vec::new())
            }
        } else {
            // No header found: keep only the last (HEADER.len() - 1) bytes in case they are
            // the start of a header, drop the rest to avoid unbounded buffer growth.
            let keep = HEADER.len().saturating_sub(1);
            if self.buffer.len() > keep {
                let start = self.buffer.len() - keep;
                let mut tail = self.buffer.split_off(start);
                self.buffer.clear();
                self.buffer.append(&mut tail);
            }
            return Ok(Vec::new());
        }
    }

    fn parse_frame(&self, frame: &[u8]) -> Result<Vec<TargetData>> {
        let target_count = frame[0] as usize;
        let _alarm_status = frame[1];

        let mut targets = Vec::new();
        let target_payload = &frame[2..];
        let record_size = 5; // 1 byte angle, 1 byte distance, 1 byte direction, 1 byte speed, 1 byte snr
        if target_payload.len() % record_size != 0 {
            return Err(anyhow::anyhow!("Frame length {} is not a multiple of record size {}", target_payload.len(), record_size));
        }

        for chunk in target_payload.chunks_exact(record_size) {
            let angle_byte = chunk[0];
            let distance = chunk[1];
            let direction_byte = chunk[2];
            let speed = chunk[3];
            let snr = chunk[4];

            let angle_i16: i16 = (angle_byte as i16) - 0x80;
            let angle_i8: i8 = angle_i16 as i8; // now in -128..127

            let direction = match direction_byte {
                0 => Direction::Receding,
                1 => Direction::Approaching,
                _ => return Err(anyhow!("Invalid direction byte: {}", direction_byte)),
            };

            targets.push(TargetData {
                angle: angle_i8,
                distance,
                direction,
                speed,
                snr,
            });
        }

        if targets.len() != target_count {
            return Err(anyhow::anyhow!("Parsed target count {} does not match reported count {}", targets.len(), target_count));
        }

        Ok(targets)
    }
}
