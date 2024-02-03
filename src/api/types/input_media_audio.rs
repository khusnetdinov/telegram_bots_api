use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inputmediaaudio
#[derive(Debug, Serialize, Deserialize)]
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
