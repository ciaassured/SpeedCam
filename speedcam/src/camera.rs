use gphoto2::{Context, Result};
use std::path::Path;

pub struct Camera {
    camera: gphoto2::Camera,
}

impl Camera {
    pub fn new() -> Result<Self> {
        let camera = Context::new()?.autodetect_camera().wait().expect("Failed to autodetect camera");
        Ok(Camera { camera })
    }

    pub fn take_photo(&self) -> Result<()> {
        let camera_fs = self.camera.fs();

        // And take pictures
        let file_path = self.camera.capture_image().wait().expect("Could not capture image");
        // download with timestamped filename
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let new_file_name = format!("{}_{}", timestamp, file_path.name());
        camera_fs.download_to(&file_path.folder(), &file_path.name(), Path::new(&new_file_name)).wait()?;

        Ok(())
    }
}
