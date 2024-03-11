use crate::api::enums::bot_commands::BotCommands;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#setmycommands>
/// Use this method to change the list of the bot's commands. See this manual for more details about bot commands. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct SetMyCommands {
    pub commands: Vec<BotCommands>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommands>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
