use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageid
/// This object represents a unique message identifier.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageId {
    message_id: i64,
}
