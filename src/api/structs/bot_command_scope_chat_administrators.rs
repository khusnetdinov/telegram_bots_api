use crate::api::enums::chat_uid::ChatUId;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#botcommandscopechatadministrators>
/// Represents the scope of bot commands, covering all administrators of a specific group or supergroup chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeChatAdministrators {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub chat_id: ChatUId,
}
