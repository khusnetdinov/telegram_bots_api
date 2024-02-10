use crate::api::types::chat::Chat;
use crate::api::types::message_id::MessageId;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageoriginchannel
/// The message was originally sent to a channel chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginChannel {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub date: i64,
    pub chat: Chat,
    pub message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}
