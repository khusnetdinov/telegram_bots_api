use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#textquote
/// This object contains information about the quoted part of a message that is replied to by the given message.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TextQuote {
    text: String,
    entities: Option<Vec<MessageEntity>>,
    position: i64,
    is_manual: Option<bool>,
}
