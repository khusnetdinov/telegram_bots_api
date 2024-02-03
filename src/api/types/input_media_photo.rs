use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inputmediaphoto
#[derive(Debug, Serialize, Deserialize)]
pub struct InputMediaPhoto {
    // type: String,
    media: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    has_spoiler: Option<bool>,
}
