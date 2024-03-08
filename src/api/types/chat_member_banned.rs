use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatmemberbanned>
/// Represents a chat member that was banned in the chat and can't return to the chat or view chat messages.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberBanned {
    pub status: String,
    pub user: User,
    pub until_date: i64,
}
