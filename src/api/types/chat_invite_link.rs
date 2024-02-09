use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatinvitelink
/// Represents an invite link for a chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatInviteLink {
    invite_link: String,
    creator: User,
    creates_join_request: bool,
    is_primary: bool,
    is_revoked: bool,
    name: Option<String>,
    expire_date: Option<i64>,
    member_limit: Option<i64>,
    pending_join_request_count: Option<i64>,
}
