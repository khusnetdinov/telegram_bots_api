use crate::api::types::chat::Chat;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inaccessiblemessage
/// This object describes a message that was deleted or is otherwise inaccessible to the bot.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InaccessibleMessage {
    chat: Chat,
    message_id: i64,
    date: i64,
}
