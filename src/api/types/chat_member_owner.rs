use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatmemberowner
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberOwner {
    status: String,
    user: User,
    is_anonymous: bool,
    custom_title: Option<String>,
}
