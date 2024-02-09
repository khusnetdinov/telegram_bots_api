use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatmemberowner
/// Represents a chat member that owns the chat and has all administrator privileges.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberOwner {
    status: String,
    user: User,
    is_anonymous: bool,
    custom_title: Option<String>,
}
