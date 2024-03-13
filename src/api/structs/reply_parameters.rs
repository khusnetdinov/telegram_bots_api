use crate::api::structs::message_entity::MessageEntity;
use crate::api::structs::message_id::MessageId;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#replyparameters>
/// Describes reply parameters for the message that is being sent.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReplyParameters {
    pub message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<i64>,
}
