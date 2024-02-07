use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommandscopechatmember
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeChatMember {
    // type: String,
    // chat_id: i64,
    user_id: i64,
}
