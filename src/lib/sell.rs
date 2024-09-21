pub fn pow_price(pow_price_factor: f64, difficulty: u32) -> f64 {
    2f64.powf(pow_price_factor * (difficulty as f64))
}

pub async fn verify_zap(_zap: String, pow_price_factor: f64, difficulty: u32) -> bool {
    let _pow_price = pow_price(pow_price_factor, difficulty);

    // todo: check zap is valid

    // let zap_value = 100000000; // todo: read zap value
    // return (zap_value as f64) > pow_price

    true
}
