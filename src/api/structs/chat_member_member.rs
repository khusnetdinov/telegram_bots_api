use crate::api::structs::user::User;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatmembermember>
/// Represents a chat member that has no additional privileges or restrictions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberMember {
    pub status: String,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}
