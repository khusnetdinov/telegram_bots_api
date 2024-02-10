use crate::api::types::callback_game::CallbackGame;
use crate::api::types::login_url::LoginUrl;
use crate::api::types::switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat;
use crate::api::types::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinekeyboardbutton
/// This object represents one button of an inline keyboard. You must use exactly one of the optional fields.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    pub web_app: Option<WebAppInfo>,
    pub login_url: Option<LoginUrl>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<bool>,
}
