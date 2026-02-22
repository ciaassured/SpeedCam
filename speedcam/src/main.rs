use anyhow::Result;
use simple_logger::SimpleLogger;
 
mod hlk_ld2451;
mod hlk_ld2415h;
mod camera;
 
fn main() -> Result<()> {
  SimpleLogger::new().init().unwrap();
 
  // let mut dist_radar = hlk_ld2451::Radar::new("/dev/serial0")?;
  let mut speed_radar = hlk_ld2415h::Radar::new("/dev/ttyAMA3")?;
  let camera = camera::Camera::new()?;
 
  loop {
    // let target_data = dist_radar.read_targets()?;
    // if !target_data.is_empty() {
    //   log::info!("Detected targets: {:?}", target_data);
    //   if target_data.iter().any(|t| t.speed > 5) {
    //     //camera.take_photo()?;
    //     dist_radar.flush()?;
    //     log::info!("Photo taken due to speed violation.");
    //   }
    // }
 
    let target = match speed_radar.read_target() {
      Ok(Some(t)) => t,
      Ok(None) => { continue; }
      Err(e) => { log::error!("Error processing speed {}", e); continue; }
    };
 
    log::debug!("Target {:?}", target);
    if target.speed > 5.0 {
      camera.take_photo()?;
      speed_radar.flush()?;
      log::info!("Photo taken due to speed violation.");
    }
  }
}
