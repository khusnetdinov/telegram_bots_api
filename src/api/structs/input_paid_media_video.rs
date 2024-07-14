use crate::api::structs::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inputpaidmediavideo>
/// The paid media to send is a video.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputPaidMediaVideo {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
}
