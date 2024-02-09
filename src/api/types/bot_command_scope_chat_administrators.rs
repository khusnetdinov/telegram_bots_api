use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#botcommandscopechatadministrators
/// Represents the scope of bot commands, covering all administrators of a specific group or supergroup chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeChatAdministrators {
    // type: String,
    chat_id: i64,
}
