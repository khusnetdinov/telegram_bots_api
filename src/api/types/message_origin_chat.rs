use crate::api::types::chat::Chat;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#messageoriginchat
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginChat {
    // type: String,
    date: i64,
    sender_chat: Chat,
    author_signature: Option<String>,
}
