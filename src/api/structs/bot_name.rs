use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#botname>
/// This object represents the bot's name.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotName {
    pub name: String,
}
