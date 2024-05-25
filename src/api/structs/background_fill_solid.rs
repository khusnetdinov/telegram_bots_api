use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#backgroundfillsolid>
/// The background is filled using the selected color.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundFillSolid {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub color: u32,
}
