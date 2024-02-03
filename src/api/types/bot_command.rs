use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommand
#[derive(Debug, Serialize, Deserialize)]
pub struct BotCommand {
    command: String,
    description: String,
}
