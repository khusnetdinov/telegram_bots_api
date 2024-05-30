use crate::api::enums::background_fill::BackgroundFill;
use crate::api::structs::document::Document;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#backgroundtypepattern
/// The background is taken directly from a built-in chat theme.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundTypePattern {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub document: Document,
    pub fill: BackgroundFill,
    pub intensity: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inverted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,
}
