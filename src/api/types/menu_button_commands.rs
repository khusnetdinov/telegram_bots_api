use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#menubuttoncommands
/// Represents a menu button, which opens the bot's list of commands.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MenuButtonCommands {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
