use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inputmediadocument
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputMediaDocument {
    // type: String,
    media: String,
    // thumbnail: Option<InputFile or String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_content_type_detection: Option<bool>,
}
