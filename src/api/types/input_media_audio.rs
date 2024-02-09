use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputmediaaudio
/// Represents an audio file to be treated as music to be sent.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputMediaAudio {
    // type: String,
    media: String,
    // thumbnail: Option<InputFile or String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    duration: Option<i64>,
    performer: Option<String>,
    title: Option<String>,
}
