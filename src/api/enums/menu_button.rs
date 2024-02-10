use crate::api::types::menu_button_commands::MenuButtonCommands;
use crate::api::types::menu_button_default::MenuButtonDefault;
use crate::api::types::menu_button_web_app::MenuButtonWebApp;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#menubutton
/// This object describes the bot's menu button in a private chat. It should be one of
/// MenuButtonCommands
/// MenuButtonWebApp
/// MenuButtonDefault
/// If a menu button other than MenuButtonDefault is set for a private chat, then it is applied in the chat. Otherwise the default menu button is applied. By default, the menu button opens the list of bot commands.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MenuButton {
    MenuButtonCommands(MenuButtonCommands),
    MenuButtonWebApp(MenuButtonWebApp),
    MenuButtonDefault(MenuButtonDefault),
}
