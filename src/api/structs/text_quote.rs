use crate::api::structs::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#textquote>
/// This object contains information about the quoted part of a message that is replied to by the given message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextQuote {
    pub text: String,
    pub position: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
}
