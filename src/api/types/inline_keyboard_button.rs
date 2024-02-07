use crate::api::types::callback_game::CallbackGame;
use crate::api::types::login_url::LoginUrl;
use crate::api::types::switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat;
use crate::api::types::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineKeyboardButton {
    text: String,
    url: Option<String>,
    callback_data: Option<String>,
    web_app: Option<WebAppInfo>,
    login_url: Option<LoginUrl>,
    switch_inline_query: Option<String>,
    switch_inline_query_current_chat: Option<String>,
    switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    callback_game: Option<CallbackGame>,
    pay: Option<bool>,
}
