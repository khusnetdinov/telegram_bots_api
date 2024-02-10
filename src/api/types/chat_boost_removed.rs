use crate::api::enums::chat_boost_source::ChatBoostSource;
use crate::api::types::chat::Chat;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatboostremoved
/// This object represents a boost removed from a chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostRemoved {
    chat: Chat,
    boost_id: String,
    remove_date: i64,
    source: ChatBoostSource,
}
