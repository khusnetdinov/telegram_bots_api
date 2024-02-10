use crate::api::types::chat::Chat;
use crate::api::types::message_id::MessageId;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageoriginchannel
/// The message was originally sent to a channel chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginChannel {
    // type: String,
    date: i64,
    chat: Chat,
    message_id: MessageId,
    author_signature: Option<String>,
}
