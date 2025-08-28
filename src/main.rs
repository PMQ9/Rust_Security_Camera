use anyhow::Result;
use opencv::{
    core::{self, Mat},
    highgui,
    prelude::*,
    videoio::{self, VideoCapture},
};

fn main() -> Result<()> {
    // Initialize the webcam capture (index 0 for default camera)
    let mut cap = VideoCapture::new(0, videoio::CAP_ANY)?;
    if !cap.is_opened()? {
        return Err(anyhow::anyhow!("Failed to open webcam"));
    }

    let window_name = "Webcam Display";
    highgui::named_window(window_name, highgui::WINDOW_AUTOSIZE)?;

    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;

        if frame.empty() {
            break;
        }

        // Perform image processing or computer vision tasks here
        // Example: Convert to grayscale (uncomment to enable)
        // let mut gray = Mat::default();
        // imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

        // Display the frame (use &gray if processing is applied)
        highgui::imshow(window_name, &frame)?;

        // Exit loop on 'Esc' key press (key code 27)
        if highgui::wait_key(1)? == 27 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}