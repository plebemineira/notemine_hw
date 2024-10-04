pub fn pow_price(pow_price_factor: f64, difficulty: u32) -> f64 {
    2f64.powf(pow_price_factor * (difficulty as f64))
}