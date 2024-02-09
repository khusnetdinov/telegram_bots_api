use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageoriginhiddenuser
/// The message was originally sent by an unknown user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginHiddenUser {
    // type: String,
    date: i64,
    sender_user_name: String,
}
