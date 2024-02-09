use crate::api::types::chat::Chat;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageoriginchannel
/// The message was originally sent to a channel chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginChannel {
    // type: String,
    date: i64,
    chat: Chat,
    message_id: i64,
    author_signature: Option<String>,
}
