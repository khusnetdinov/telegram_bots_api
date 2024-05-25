use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#backgroundfillgradient>
/// The background is a gradient fill.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundFillGradient {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub top_color: u32,
    pub bottom_color: u32,
    pub rotation_angle: u16,
}
