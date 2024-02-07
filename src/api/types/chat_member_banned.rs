use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatmemberbanned
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberBanned {
    status: String,
    user: User,
    until_date: i64,
}
