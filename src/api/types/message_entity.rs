use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#messageentity
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEntity {
    // type: String,
    offset: i64,
    length: i64,
    url: Option<String>,
    user: Option<User>,
    language: Option<String>,
    custom_emoji_id: Option<String>,
}
