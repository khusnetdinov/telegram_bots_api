use crate::api::enums::chat_boost_source::ChatBoostSource;
use crate::api::structs::chat::Chat;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatboostremoved>
/// This object represents a boost removed from a chat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostRemoved {
    pub chat: Chat,
    pub boost_id: String,
    pub remove_date: i64,
    pub source: ChatBoostSource,
}
