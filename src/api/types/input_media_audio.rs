use crate::api::enums::file_input::FileInput;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputmediaaudio
/// Represents an audio file to be treated as music to be sent.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputMediaAudio {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub thumbnail: Option<FileInput>,
    pub media: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<i64>,
    pub performer: Option<String>,
    pub title: Option<String>,
}
