use crate::error::ZapError;

pub fn pow_price(pow_price_factor: f64, difficulty: u32) -> f64 {
    2f64.powf(pow_price_factor * (difficulty as f64))
}

pub async fn verify_zap(_zap: String, pow_price_factor: f64, difficulty: u32) -> Result<(), ZapError> {
    let _pow_price = pow_price(pow_price_factor, difficulty);

    // todo: check zap is valid
    // if !valid_zap {
    //   return Err(ZapError::InvalidZap);
    // }

    // todo: read zap value
    // if (zap_value as f64) < pow_price {
    //   return Err(ZapError::InsufficientZap);
    // }

    Ok(())
}
