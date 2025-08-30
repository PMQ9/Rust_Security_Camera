use anyhow::Result;
use opencv::{
    core::{self, Mat, Point, Rect, Scalar},
    highgui,
    imgproc,
    prelude::*,
    videoio::{self, VideoCapture},
};
use std::time::{Duration, Instant};

// Expected LED patterns from led_controller.rs
const LED1_PATTERN: [u8; 4] = [0, 0, 1, 0]; // ACT LED pattern
const LED2_PATTERN: [u8; 4] = [0, 1, 1, 0]; // PWR LED pattern
const PATTERN_LENGTH: usize = 4;
const SAMPLE_INTERVAL: Duration = Duration::from_millis(1000); // 1-second sampling to match LED timing
const BRIGHTNESS_THRESHOLD: f64 = 100.0; // Threshold to determine if LED is on (adjust as needed)

fn main() -> Result<()> {
    // Initialize the webcam capture (index 0 for default webcam)
    let mut cap = VideoCapture::new(0, videoio::CAP_ANY)?;
    if !cap.is_opened()? {
        return Err(anyhow::anyhow!("Failed to open webcam"));
    }

    let window_name = "Webcam Display";
    highgui::named_window(window_name, highgui::WINDOW_AUTOSIZE)?;

    // Buffers to store detected LED states
    let mut led1_states: Vec<u8> = Vec::with_capacity(PATTERN_LENGTH);
    let mut led2_states: Vec<u8> = Vec::with_capacity(PATTERN_LENGTH);
    let mut last_sample_time = Instant::now();

    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;

        if frame.empty() {
            break;
        }

        // Get frame dimensions
        let width = frame.cols();
        let height = frame.rows();

        // Define the main ROI (top-right corner, 10% of frame size)
        let rect_width = (width as f32 * 0.1) as i32; // 10% of frame width
        let rect_height = (height as f32 * 0.1) as i32; // 10% of frame height
        let top_left_x = width - rect_width - 10; // 10-pixel margin from right
        let top_left_y = 10; // 10-pixel margin from top

        // Split the main ROI into two rectangles: ROI1 (left) for LED1, ROI2 (right) for LED2
        let roi1 = Rect::new(top_left_x, top_left_y, rect_width / 2, rect_height); // Left half for LED1
        let roi2 = Rect::new(top_left_x + rect_width / 2, top_left_y, rect_width / 2, rect_height); // Right half for LED2

        // Extract ROI1 and ROI2 from the frame
        let led1_region = Mat::roi(&frame, roi1)?;
        let led2_region = Mat::roi(&frame, roi2)?;

        // Sample LED states at 1-second intervals
        if last_sample_time.elapsed() >= SAMPLE_INTERVAL {
            // Compute average brightness for each LED region
            let led1_brightness = core::mean(&led1_region, &Mat::default())?.0[0]; // Use first channel (BGR)
            let led2_brightness = core::mean(&led2_region, &Mat::default())?.0[0];

            // Determine LED states based on brightness threshold
            let led1_state = if led1_brightness > BRIGHTNESS_THRESHOLD { 1 } else { 0 };
            let led2_state = if led2_brightness > BRIGHTNESS_THRESHOLD { 1 } else { 0 };

            // Store states
            led1_states.push(led1_state);
            led2_states.push(led2_state);

            // Keep only the last PATTERN_LENGTH states
            if led1_states.len() > PATTERN_LENGTH {
                led1_states.remove(0);
            }
            if led2_states.len() > PATTERN_LENGTH {
                led2_states.remove(0);
            }

            // Reset sampling time
            last_sample_time = Instant::now();
        }

        // Check if patterns match
        let is_verified = if led1_states.len() == PATTERN_LENGTH && led2_states.len() == PATTERN_LENGTH {
            led1_states == LED1_PATTERN && led2_states == LED2_PATTERN
        } else {
            false
        };

        // Draw ROI1 rectangle (red, thickness 2) for LED1
        imgproc::rectangle(
            &mut frame,
            roi1,
            Scalar::new(0.0, 0.0, 255.0, 0.0), // Red color in BGR
            2,
            imgproc::LINE_8,
            0,
        )?;

        // Draw ROI2 rectangle (blue, thickness 2) for LED2
        imgproc::rectangle(
            &mut frame,
            roi2,
            Scalar::new(255.0, 0.0, 0.0, 0.0), // Blue color in BGR
            2,
            imgproc::LINE_8,
            0,
        )?;

        // Display verification status
        let text = if is_verified {
            "Footage verified"
        } else {
            "Footage not verified"
        };
        imgproc::put_text(
            &mut frame,
            text,
            Point::new(10, 30), // Position near top-left
            imgproc::FONT_HERSHEY_SIMPLEX,
            0.7, // Font scale
            Scalar::new(0.0, 255.0, 0.0, 0.0), // Green text
            2,
            imgproc::LINE_8,
            false,
        )?;

        // Display the frame
        highgui::imshow(window_name, &frame)?;

        // Exit loop on 'Esc' key press (key code 27)
        if highgui::wait_key(1)? == 27 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}