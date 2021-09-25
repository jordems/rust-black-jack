
use rand::{Rng};


/// Generates a crypto graphically secure random number
/// 
/// This function only supports u8 number ranges (0-255)
pub fn generate_random_number(start: u8, end: u8) -> u8 {
    let mut rng = rand::thread_rng();

    rng.gen_range(start..end)
}

