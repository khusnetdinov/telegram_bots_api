use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#botcommand
/// This object represents a bot command.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommand {
    command: String,
    description: String,
}
