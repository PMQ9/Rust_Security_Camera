use anyhow::Result;
use opencv::{
    core::{self, Mat, Point, Rect, Scalar},
    highgui,
    imgproc,
    prelude::*,
    videoio::{self, VideoCapture},
};

fn main() -> Result<()> {
    // Initialize the webcam capture (index 0 for default webcam, index 1 for external camera)
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

        // Get frame dimensions
        let width = frame.cols();
        let height = frame.rows();

        // Define the rectangle's top-left and bottom-right corners (top-right corner, 10% of frame size)
        let rect_width = (width as f32 * 0.1) as i32; // 10% of frame width
        let rect_height = (height as f32 * 0.1) as i32; // 10% of frame height
        let top_left_x = width - rect_width - 10; // 10-pixel margin from right
        let top_left_y = 10; // 10-pixel margin from top

        // Create a Rect for the rectangle
        let rect = Rect::new(
            top_left_x,
            top_left_y,
            rect_width,
            rect_height,
        );

        // Draw a red rectangle (color in BGR: (0, 0, 255), thickness: 2)
        imgproc::rectangle(
            &mut frame,
            rect,
            Scalar::new(0.0, 0.0, 255.0, 0.0), // Red color in BGR
            2, // Thickness
            imgproc::LINE_8,
            0,
        )?;

        // Display the frame with the rectangle
        highgui::imshow(window_name, &frame)?;

        // Exit loop on 'Esc' key press (key code 27)
        if highgui::wait_key(1)? == 27 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}