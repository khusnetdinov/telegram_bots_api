use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#botcommandscopeallprivatechats
/// Represents the scope of bot commands, covering all private chats.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeAllPrivateChats {
    // type: String,
}
