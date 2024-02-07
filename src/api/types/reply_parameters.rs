use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#replyparameters
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReplyParameters {
    message_id: i64,
    chat_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    quote: Option<String>,
    quote_parse_mode: Option<String>,
    quote_entities: Option<Vec<MessageEntity>>,
    quote_position: Option<i64>,
}
