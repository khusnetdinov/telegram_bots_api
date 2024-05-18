use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#maskposition>
/// This object describes the position on faces where a mask should be placed by default.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}
