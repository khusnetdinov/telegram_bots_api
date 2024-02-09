use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageoriginuser
/// The message was originally sent by a known user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginUser {
    // type: String,
    date: i64,
    sender_user: User,
}
