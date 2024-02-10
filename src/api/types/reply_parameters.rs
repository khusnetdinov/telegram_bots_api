use crate::api::types::message_entity::MessageEntity;
use crate::api::types::message_id::MessageId;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#replyparameters
/// Describes reply parameters for the message that is being sent.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReplyParameters {
    pub message_id: MessageId,
    pub chat_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub quote: Option<String>,
    pub quote_parse_mode: Option<String>,
    pub quote_entities: Option<Vec<MessageEntity>>,
    pub quote_position: Option<i64>,
}
