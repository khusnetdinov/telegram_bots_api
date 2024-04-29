use crate::api::structs::user::User;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#businessconnection>
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BusinessConnection {
    pub id: String,
    pub user: User,
    pub user_chat_id: u64,
    pub date: u64,
    pub can_reply: bool,
    pub is_enabled: bool,
}
