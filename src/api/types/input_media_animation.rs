use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inputmediaanimation
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputMediaAnimation {
    // type: String,
    media: String,
    // thumbnail: Option<InputFile or String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    width: Option<i64>,
    height: Option<i64>,
    duration: Option<i64>,
    has_spoiler: Option<bool>,
}
