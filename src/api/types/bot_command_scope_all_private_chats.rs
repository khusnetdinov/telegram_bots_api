use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommandscopeallprivatechats
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeAllPrivateChats {
    // type: String,
}
