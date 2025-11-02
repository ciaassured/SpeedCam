use anyhow::Result;
use simple_logger::SimpleLogger;

mod hlk_ld2451;
mod camera;

fn main() -> Result<()> {
  SimpleLogger::new().init().unwrap();

  let mut radar = hlk_ld2451::Radar::new("/dev/serial0")?;
  let camera = camera::Camera::new()?;

  loop {
      let data = radar.read_targets()?;
      if !data.is_empty() {
        log::info!("Detected targets: {:?}", data);
        if data.iter().any(|t| t.speed > 5) {
          camera.take_photo()?;
          radar.flush()?;
          log::info!("Photo taken due to speed violation.");
        }
      }
  }
}
