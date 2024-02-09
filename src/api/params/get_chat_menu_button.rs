use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#getchatmenubutton
/// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns MenuButton on success.
#[derive(Debug, Serialize)]
pub struct GetChatMenuButton {
    chat_id: Option<ChatUId>,
}
