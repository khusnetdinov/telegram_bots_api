use crate::api::structs::document::Document;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#backgroundtypewallpaper>
/// The background is a wallpaper in the JPEG format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundTypeWallpaper {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub document: Document,
    pub dark_theme_dimming: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blurred: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,
}
