use crate::api::enums::chat_uid::ChatUId;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#botcommandscopechatmember>
/// Represents the scope of bot commands, covering a specific member of a group or supergroup chat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeChatMember {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub chat_id: ChatUId,
    pub user_id: i64,
}
