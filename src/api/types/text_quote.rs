use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#textquote
#[derive(Debug, Serialize, Deserialize)]
pub struct TextQuote {
    text: String,
    entities: Option<Vec<MessageEntity>>,
    position: i64,
    is_manual: Option<bool>,
}
