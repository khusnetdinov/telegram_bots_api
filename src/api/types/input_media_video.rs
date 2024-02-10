use crate::api::enums::file_input::FileInput;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputmediavideo
/// Represents a video to be sent.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputMediaVideo {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub thumbnail: Option<FileInput>,
    pub caption: Option<String>,
    pub media: String,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration: Option<i64>,
    pub supports_streaming: Option<bool>,
    pub has_spoiler: Option<bool>,
}
