use crate::api::structs::user::User;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatmemberleft>
/// Represents a chat member that isn't currently a member of the chat, but may join it themselves.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberLeft {
    pub status: String,
    pub user: User,
}
