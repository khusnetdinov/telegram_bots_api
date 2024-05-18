use crate::api::structs::chat_boost::ChatBoost;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#userchatboosts>
/// This object represents a list of boosts added to a chat by a user.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserChatBoosts {
    pub boosts: Vec<ChatBoost>,
}
