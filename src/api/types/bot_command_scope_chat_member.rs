use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#botcommandscopechatmember
/// Represents the scope of bot commands, covering a specific member of a group or supergroup chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeChatMember {
    // type: String,
    // chat_id: i64,
    user_id: i64,
}
