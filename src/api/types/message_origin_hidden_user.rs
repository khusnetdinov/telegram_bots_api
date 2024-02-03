use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#messageoriginhiddenuser
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageOriginHiddenUser {
    // type: String,
    date: i64,
    sender_user_name: String,
}
