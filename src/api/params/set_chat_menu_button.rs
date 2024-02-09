use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::menu_button::MenuButton;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setchatmenubutton
/// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetChatMenuButton {
    chat_id: Option<ChatUId>,
    menu_button: Option<MenuButton>,
}
