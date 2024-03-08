use crate::api::types::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#menubutton>
/// This object describes the bot's menu button in a private chat. It should be one of
/// MenuButtonCommands
/// MenuButtonWebApp
/// MenuButtonDefault
/// If a menu button other than MenuButtonDefault is set for a private chat, then it is applied in the chat. Otherwise the default menu button is applied. By default, the menu button opens the list of bot commands.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MenuButton {
    Commands,
    WebApp { text: String, web_app: WebAppInfo },
    Default,
}
