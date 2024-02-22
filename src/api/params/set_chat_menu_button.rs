use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::menu_button::MenuButton;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setchatmenubutton
/// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct SetChatMenuButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatUId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<MenuButton>,
}
