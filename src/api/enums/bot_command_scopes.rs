use crate::api::structs::bot_command_scope_all_chat_administrators::BotCommandScopeAllChatAdministrators;
use crate::api::structs::bot_command_scope_all_group_chats::BotCommandScopeAllGroupChats;
use crate::api::structs::bot_command_scope_all_private_chats::BotCommandScopeAllPrivateChats;
use crate::api::structs::bot_command_scope_chat::BotCommandScopeChat;
use crate::api::structs::bot_command_scope_chat_administrators::BotCommandScopeChatAdministrators;
use crate::api::structs::bot_command_scope_chat_member::BotCommandScopeChatMember;
use crate::api::structs::bot_command_scope_default::BotCommandScopeDefault;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#botcommandscope>
/// This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:
/// BotCommandScopeDefault
/// BotCommandScopeAllPrivateChats
/// BotCommandScopeAllGroupChats
/// BotCommandScopeAllChatAdministrators
/// BotCommandScopeChat
/// BotCommandScopeChatAdministrators
/// BotCommandScopeChatMember
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum BotCommandScopes {
    Default(BotCommandScopeDefault),
    AllPrivateChats(BotCommandScopeAllPrivateChats),
    AllGroupChats(BotCommandScopeAllGroupChats),
    AllChatAdministrators(BotCommandScopeAllChatAdministrators),
    Chat(BotCommandScopeChat),
    ChatAdministrators(BotCommandScopeChatAdministrators),
    ChatMember(BotCommandScopeChatMember),
}
