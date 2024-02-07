use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommandscope
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScope {}
