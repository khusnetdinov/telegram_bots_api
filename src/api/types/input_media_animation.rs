use crate::api::enums::file_input::FileInput;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputmediaanimation
/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputMediaAnimation {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub thumbnail: Option<FileInput>,
    pub media: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration: Option<i64>,
    pub has_spoiler: Option<bool>,
}
