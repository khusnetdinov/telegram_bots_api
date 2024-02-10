use crate::api::enums::bot_command_scope::BotCommandScope;
use serde::Serialize;

/// https://core.telegram.org/bots/api#deletemycommands
/// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, higher level commands will be shown to affected users. Returns True on success.
#[derive(Debug, Serialize)]
pub struct DeleteMyCommands {
    pub scope: Option<BotCommandScope>,
    pub language_code: Option<String>,
}
