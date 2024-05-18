use crate::api::structs::chat::Chat;
use crate::api::structs::message_id::MessageId;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inaccessiblemessage>
/// This object describes a message that was deleted or is otherwise inaccessible to the bot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InaccessibleMessage {
    pub chat: Chat,
    pub message_id: MessageId,
    pub date: i64,
}
