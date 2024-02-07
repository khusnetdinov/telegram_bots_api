use crate::api::types::chat::Chat;
use crate::api::types::chat_boost_source::ChatBoostSource;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatboostremoved
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostRemoved {
    chat: Chat,
    boost_id: String,
    remove_date: i64,
    source: ChatBoostSource,
}
