use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#botcommandscope
/// This object represents the scope to which bot commands are applied.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScope {}
