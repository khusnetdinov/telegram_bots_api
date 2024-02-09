use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#maskposition
/// This object describes the position on faces where a mask should be placed by default.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MaskPosition {
    point: String,
    x_shift: f64,
    y_shift: f64,
    scale: f64,
}
