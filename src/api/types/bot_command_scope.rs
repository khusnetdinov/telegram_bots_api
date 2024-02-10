use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#botcommandscope
/// This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:
/// BotCommandScopeDefault
/// BotCommandScopeAllPrivateChats
/// BotCommandScopeAllGroupChats
/// BotCommandScopeAllChatAdministrators
/// BotCommandScopeChat
/// BotCommandScopeChatAdministrators
/// BotCommandScopeChatMember
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScope {}
