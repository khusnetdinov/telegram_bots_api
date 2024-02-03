use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommandscopeallprivatechats
#[derive(Debug, Serialize, Deserialize)]
pub struct BotCommandScopeAllPrivateChats {
    // type: String,
}
