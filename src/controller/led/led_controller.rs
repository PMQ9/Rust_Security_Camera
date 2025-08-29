// Output security patternuse std::fs::OpenOptions;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::fs::OpenOptions;

fn main() -> io::Result<()> {
    let act_path = "/sys/class/leds/ACT/brightness";
    let pwr_path = "/sys/class/leds/PWR/brightness";

    let led1_pattern: [u8; 4] = [0, 0, 1, 0];
    let led2_pattern: [u8; 4] = [0, 1, 1, 0];

    loop {
        for i in 0..led1_pattern.len() {
            write_to_led(act_path, led1_pattern[i])?;
            write_to_led(pwr_path, led2_pattern[i])?;
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn write_to_led(path: &str, value: u8) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Failed to open LED brightness file");

    file.write_all(value.to_string().as_bytes())
        .expect("Failed to write to LED brightness file");

    file.flush()
        .expect("Failed to flush changes to LED brightness file");

    Ok(())
}