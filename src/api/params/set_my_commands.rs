use crate::api::enums::bot_command_scope::BotCommandScope;
use crate::api::types::bot_command::BotCommand;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setmycommands
/// Use this method to change the list of the bot's commands. See this manual for more details about bot commands. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetMyCommands {
    commands: Vec<BotCommand>,
    scope: Option<BotCommandScope>,
    language_code: Option<String>,
}
