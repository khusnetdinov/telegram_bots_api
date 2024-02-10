use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatshared
/// This object contains information about the chat whose identifier was shared with the bot using a KeyboardButtonRequestChat button.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatShared {
    pub request_id: i64,
    pub chat_id: i64,
}
