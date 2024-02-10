use crate::api::enums::file_input::FileInput;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputmediadocument
/// Represents a general file to be sent.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputMediaDocument {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub thumbnail: Option<FileInput>,
    pub caption: Option<String>,
    pub media: String,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_content_type_detection: Option<bool>,
}
