use crate::api::types::bot_command_scope::BotCommandScope;
use serde::Serialize;

/// https://core.telegram.org/bots/api#getmycommands
/// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of BotCommand objects. If commands aren't set, an empty list is returned.
#[derive(Debug, Serialize)]
pub struct GetMyCommands {
    scope: Option<BotCommandScope>,
    language_code: Option<String>,
}
