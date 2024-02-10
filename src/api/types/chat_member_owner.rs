use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatmemberowner
/// Represents a chat member that owns the chat and has all administrator privileges.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberOwner {
    pub status: String,
    pub user: User,
    pub is_anonymous: bool,
    pub custom_title: Option<String>,
}
