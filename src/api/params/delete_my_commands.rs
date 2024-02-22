use crate::api::enums::bot_command_scope::BotCommandScope;
use serde::Serialize;

/// https://core.telegram.org/bots/api#deletemycommands
/// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, higher level commands will be shown to affected users. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct DeleteMyCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
