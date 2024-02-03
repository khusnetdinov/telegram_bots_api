use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommandscopechatadministrators
#[derive(Debug, Serialize, Deserialize)]
pub struct BotCommandScopeChatAdministrators {
    // type: String,
    chat_id: i64,
}
