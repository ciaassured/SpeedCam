use std::{time::Duration};
 
use anyhow::{Context, Result, anyhow};
 
 
pub struct Radar {
    port: Box<dyn serialport::SerialPort>,
}
 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Approaching,
    Receding,
}
 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TargetData {
    pub direction: Direction,
    pub speed: f32,
}
 
impl Radar {
    pub fn new(port_name: &str) -> Result<Self> {
        let port = serialport::new(port_name, 9600)
            .timeout(Duration::from_millis(10))
            .open()
            .context("Failed to open HLK-LD2415H serial port")?;
 
        log::debug!("Successfully connected to HLK-LD2415H on port {}", port_name);
 
        Ok(Radar { port })
    }
 
    pub fn flush(&mut self) -> Result<()> {
        self.port.clear(serialport::ClearBuffer::Input).context("Failed to flush HLK-LD2415H input buffer")?;
        Ok(())
    }
 
    /// Read speed and direction of current radar target.
    pub fn read_target(&mut self) -> Result<Option<TargetData>> {
        // Read any available bytes and append to internal buffer
        let available = self.port.bytes_to_read()?;
        if available > 0 {
            let mut tmp = vec![0u8; available as usize];
            let bytes_read = self.port.read(&mut tmp).context("Failed to read from HLK-LD2415H")?;
            tmp.truncate(bytes_read);
 
            let target_str = str::from_utf8(&tmp).context("Invalid string form HLK-LD2415H")?;
            let target = Self::parse(target_str)?;
            Ok(Some(target))
        } else {
            Ok(None)
        }
    }
 
    fn parse(input: &str) -> Result<TargetData> {
        if input.len() != 9 {
            return Err(anyhow!("Invalid data length from HLK-LD2415H"));
        }
 
        let sign = &input[1..2];
        let number = &input[2..];
 
        let speed: f32 = number.trim().parse().context("Failed to parse speed value from HLK-LD2415H")?;
        let direction = match sign {
            "+" => Direction::Approaching,
            "-" => Direction::Receding,
            _ => { return Err(anyhow!("Invalid data length from HLK-LD2415H")); }
        };
 
        Ok(TargetData { direction, speed })
    }
}
