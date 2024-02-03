use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#maskposition
#[derive(Debug, Serialize, Deserialize)]
pub struct MaskPosition {
    point: String,
    x_shift: f64,
    y_shift: f64,
    scale: f64,
}
