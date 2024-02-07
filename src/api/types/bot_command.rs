use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommand
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommand {
    command: String,
    description: String,
}
