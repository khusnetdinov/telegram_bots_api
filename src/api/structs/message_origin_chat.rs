use crate::api::structs::chat::Chat;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#messageoriginchat>
/// The message was originally sent on behalf of a chat to a group chat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginChat {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub date: i64,
    pub sender_chat: Chat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}
