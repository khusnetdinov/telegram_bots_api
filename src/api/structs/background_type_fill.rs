use crate::api::enums::background_fill::BackgroundFill;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#backgroundtypefill>
/// The background is automatically filled based on the selected colors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundTypeFill {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub fill: BackgroundFill,
    pub dark_theme_dimming: u8,
}
