use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatshared
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatShared {
    request_id: i64,
    chat_id: i64,
}
