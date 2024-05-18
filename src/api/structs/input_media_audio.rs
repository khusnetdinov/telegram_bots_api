use crate::api::enums::file_input::FileInput;
use crate::api::structs::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inputmediaaudio>
/// Represents an audio file to be treated as music to be sent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InputMediaAudio {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
