use crate::api::enums::file_input::FileInput;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputmediadocument
/// Represents a general file to be sent.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct InputMediaDocument {
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
    pub disable_content_type_detection: Option<bool>,
}
