use crate::api::types::chat_administrator_rights::ChatAdministratorRights;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#keyboardbuttonrequestchat
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KeyboardButtonRequestChat {
    request_id: i64,
    chat_is_channel: bool,
    chat_is_forum: Option<bool>,
    chat_has_username: Option<bool>,
    chat_is_created: Option<bool>,
    user_administrator_rights: Option<ChatAdministratorRights>,
    bot_administrator_rights: Option<ChatAdministratorRights>,
    bot_is_member: Option<bool>,
}
