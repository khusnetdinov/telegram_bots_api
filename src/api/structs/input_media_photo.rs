use crate::api::structs::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inputmediaphoto>
/// Represents a photo to be sent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InputMediaPhoto {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}
