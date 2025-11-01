use gphoto2::{Context, Result};
use std::path::Path;

fn main() -> Result<()> {
  // Create a new context and detect the first camera from it
  let camera = Context::new()?.autodetect_camera().wait().expect("Failed to autodetect camera");
  let camera_fs = camera.fs();


  // And take pictures
  let file_path = camera.capture_image().wait().expect("Could not capture image");
  camera_fs.download_to(&file_path.folder(), &file_path.name(), Path::new(&file_path.name().to_string())).wait()?;

  // For more advanced examples take a look at the examples/ folder

  Ok(())
}