use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatshared
/// This object contains information about the chat whose identifier was shared with the bot using a KeyboardButtonRequestChat button.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatShared {
    request_id: i64,
    chat_id: i64,
}
