use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#messageoriginhiddenuser>
/// The message was originally sent by an unknown user.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginHiddenUser {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub date: i64,
    pub sender_user_name: String,
}
