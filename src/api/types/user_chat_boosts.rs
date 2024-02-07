use crate::api::types::chat_boost::ChatBoost;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#userchatboosts
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserChatBoosts {
    boosts: Vec<ChatBoost>,
}
