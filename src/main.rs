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
const CALIBRATION_SAMPLES: usize = 10; // Number of samples for calibration
const MIN_BRIGHTNESS_DIFF: f64 = 2.0; // Minimum difference between max and min brightness
const VERIFICATION_HOLD_DURATION: Duration = Duration::from_millis(3500); // Hold verified status for 3 seconds

fn calibrate_thresholds(cap: &mut VideoCapture, roi1: Rect, roi2: Rect, window_name: &str) -> Result<(f64, f64)> {
    let mut led1_brightnesses = Vec::with_capacity(CALIBRATION_SAMPLES);
    let mut led2_brightnesses = Vec::with_capacity(CALIBRATION_SAMPLES);

    println!("Starting calibration phase...");

    loop {
        led1_brightnesses.clear();
        led2_brightnesses.clear();

        // Collect 10 samples
        for i in 0..CALIBRATION_SAMPLES {
            let mut frame = Mat::default();
            cap.read(&mut frame)?;
            if frame.empty() {
                return Err(anyhow::anyhow!("Failed to capture frame during calibration"));
            }

            let led1_region = Mat::roi(&frame, roi1)?;
            let led2_region = Mat::roi(&frame, roi2)?;

            let led1_brightness = core::mean(&led1_region, &Mat::default())?.0[0];
            let led2_brightness = core::mean(&led2_region, &Mat::default())?.0[0];

            led1_brightnesses.push(led1_brightness);
            led2_brightnesses.push(led2_brightness);

            println!(
                "Calibration sample {}: LED1 Brightness = {:.2}, LED2 Brightness = {:.2}",
                i + 1,
                led1_brightness,
                led2_brightness
            );

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

            // Display calibration status
            let text = format!("Calibration in progress: Sample {}/{}", i + 1, CALIBRATION_SAMPLES);
            imgproc::put_text(
                &mut frame,
                &text,
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

            // Check for 'Esc' key to exit calibration
            if highgui::wait_key(1)? == 27 {
                return Err(anyhow::anyhow!("Calibration interrupted by user (Esc key)"));
            }

            std::thread::sleep(SAMPLE_INTERVAL);
        }

        // Calculate max, min, and threshold for each LED
        let led1_max = led1_brightnesses.iter().fold(f64::MIN, |a, &b| a.max(b));
        let led1_min = led1_brightnesses.iter().fold(f64::MAX, |a, &b| a.min(b));
        let led2_max = led2_brightnesses.iter().fold(f64::MIN, |a, &b| a.max(b));
        let led2_min = led2_brightnesses.iter().fold(f64::MAX, |a, &b| a.min(b));

        let led1_diff = led1_max - led1_min;
        let led2_diff = led2_max - led2_min;

        println!(
            "LED1: Max = {:.2}, Min = {:.2}, Diff = {:.2}",
            led1_max, led1_min, led1_diff
        );
        println!(
            "LED2: Max = {:.2}, Min = {:.2}, Diff = {:.2}",
            led2_max, led2_min, led2_diff
        );

        // Check if differences meet the minimum requirement
        if led1_diff >= MIN_BRIGHTNESS_DIFF && led2_diff >= MIN_BRIGHTNESS_DIFF {
            let led1_threshold = (led1_max + led1_min) / 2.0;
            let led2_threshold = (led2_max + led2_min) / 2.0;
            println!(
                "Calibration successful: LED1 Threshold = {:.2}, LED2 Threshold = {:.2}",
                led1_threshold, led2_threshold
            );
            return Ok((led1_threshold, led2_threshold));
        } else {
            println!(
                "Calibration failed: LED1 Diff = {:.2}, LED2 Diff = {:.2}. Retrying...",
                led1_diff, led2_diff
            );
        }
    }
}

fn main() -> Result<()> {
    // Initialize the webcam capture (index 1 as specified)
    let mut cap = VideoCapture::new(1, videoio::CAP_ANY)?;
    if !cap.is_opened()? {
        return Err(anyhow::anyhow!("Failed to open webcam"));
    }

    let window_name = "Webcam Display";
    highgui::named_window(window_name, highgui::WINDOW_AUTOSIZE)?;

    // Get frame dimensions for ROI setup
    let mut frame = Mat::default();
    cap.read(&mut frame)?;
    if frame.empty() {
        return Err(anyhow::anyhow!("Failed to capture initial frame"));
    }
    let width = frame.cols();
    let height = frame.rows();

    // Define the main ROI (top-right corner, 10% of frame size)
    let rect_width = (width as f32 * 0.1) as f32; // 10% of frame width
    let rect_height = (height as f32 * 0.1) as f32; // 10% of frame height
    let top_left_x = width as f32 - rect_width - 10.0; // 10-pixel margin from right
    let top_left_y = 10.0; // 10-pixel margin from top

    // Split the main ROI into two rectangles: ROI1 (left) for LED1, ROI2 (right) for LED2
    let roi1 = Rect::new(top_left_x as i32, top_left_y as i32, (rect_width / 2.0) as i32, rect_height as i32); // Left half for LED1
    let roi2 = Rect::new(
        (top_left_x + rect_width / 2.0) as i32,
        top_left_y as i32,
        (rect_width / 2.0) as i32,
        rect_height as i32,
    ); // Right half for LED2

    // Perform calibration to determine thresholds
    let (brightness_threshold_led1, brightness_threshold_led2) = calibrate_thresholds(&mut cap, roi1, roi2, window_name)?;

    // Buffers to store detected LED states
    let mut led1_states: Vec<u8> = Vec::with_capacity(PATTERN_LENGTH);
    let mut led2_states: Vec<u8> = Vec::with_capacity(PATTERN_LENGTH);
    let mut last_sample_time = Instant::now();
    let mut is_verified = false;
    let mut last_verified_time: Option<Instant> = None;

    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;

        if frame.empty() {
            break;
        }

        // Extract ROI1 and ROI2 from the frame
        let led1_region = Mat::roi(&frame, roi1)?;
        let led2_region = Mat::roi(&frame, roi2)?;

        // Sample LED states at 1-second intervals
        if last_sample_time.elapsed() >= SAMPLE_INTERVAL {
            // Compute average brightness for each LED region
            let led1_brightness = core::mean(&led1_region, &Mat::default())?.0[0]; // Use first channel (BGR)
            let led2_brightness = core::mean(&led2_region, &Mat::default())?.0[0];

            // Determine LED states based on brightness threshold
            let led1_state = if led1_brightness > brightness_threshold_led1 { 1 } else { 0 };
            let led2_state = if led2_brightness > brightness_threshold_led2 { 1 } else { 0 };

            // Print brightness and state for each LED
            println!(
                "LED1 (ROI1): Brightness = {:.2}, State = {} ({})",
                led1_brightness,
                led1_state,
                if led1_state == 1 { "ON" } else { "OFF" }
            );
            println!(
                "LED2 (ROI2): Brightness = {:.2}, State = {} ({})",
                led2_brightness,
                led2_state,
                if led2_state == 1 { "ON" } else { "OFF" }
            );

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

            // Check if patterns match, but only update verification status if not in hold period
            if last_verified_time.map_or(true, |t| t.elapsed() >= VERIFICATION_HOLD_DURATION) {
                if led1_states.len() == PATTERN_LENGTH && led2_states.len() == PATTERN_LENGTH {
                    is_verified = led1_states == LED1_PATTERN && led2_states == LED2_PATTERN;
                    if is_verified {
                        last_verified_time = Some(Instant::now());
                    } else {
                        last_verified_time = None;
                    }
                } else {
                    is_verified = false;
                    last_verified_time = None;
                }
            }

            // Reset sampling time
            last_sample_time = Instant::now();
        }

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