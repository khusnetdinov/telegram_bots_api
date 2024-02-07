use crate::api::types::chat::Chat;
use crate::api::types::chat_boost::ChatBoost;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatboostupdated
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostUpdated {
    chat: Chat,
    boost: ChatBoost,
}
