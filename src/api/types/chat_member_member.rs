use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatmembermember>
/// Represents a chat member that has no additional privileges or restrictions.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberMember {
    pub status: String,
    pub user: User,
}
