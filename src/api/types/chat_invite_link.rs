use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatinvitelink
/// Represents an invite link for a chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub name: Option<String>,
    pub expire_date: Option<i64>,
    pub member_limit: Option<i64>,
    pub pending_join_request_count: Option<i64>,
}
