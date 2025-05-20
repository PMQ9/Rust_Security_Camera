use rand::rngs::OsRng;
use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PatternConfig {
    pub length: usize,
    pub min_digit: u8,
    pub max_digit: u8,
    pub repeat_allowed: bool,
}

impl Default for PatternConfig {
    fn default() -> Self {
        Self {
            length: 10,
            min_digit: 0,
            max_digit: 9,
            repeat_allowed: true,
        }
    }
}

pub fn generate_pattern(config: &PatternConfig) -> Vec<u8> {
    let mut rng = OsRng;
    let mut pattern = Vec::with_capacity(config.length);

    while pattern.len() < config.length {
        let digit = rng.gen_range(config.min_digit..=config.max_digit);
        if config.repeat_allowed || !pattern.contains(&digit) {
            pattern.push(digit);
        }
    }

    pattern
}
