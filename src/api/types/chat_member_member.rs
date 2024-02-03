use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatmembermember
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMemberMember {
    status: String,
    user: User,
}
