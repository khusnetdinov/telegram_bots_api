use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatmemberleft
/// Represents a chat member that isn't currently a member of the chat, but may join it themselves.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberLeft {
    status: String,
    user: User,
}
