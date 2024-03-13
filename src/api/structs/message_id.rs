use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#messageid>
/// This object represents a unique message identifier.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct MessageId {
    pub message_id: i64,
}

impl From<i64> for MessageId {
    fn from(message_id: i64) -> Self {
        Self { message_id }
    }
}
