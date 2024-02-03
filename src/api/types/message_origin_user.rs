use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#messageoriginuser
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageOriginUser {
    // type: String,
    date: i64,
    sender_user: User,
}
