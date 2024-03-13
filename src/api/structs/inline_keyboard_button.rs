use crate::api::structs::callback_game::CallbackGame;
use crate::api::structs::login_url::LoginUrl;
use crate::api::structs::switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat;
use crate::api::structs::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inlinekeyboardbutton>
/// This object represents one button of an inline keyboard. You must use exactly one of the optional fields.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineKeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}
