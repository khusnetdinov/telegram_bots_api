use crate::api::types::keyboard_button_poll_type::KeyboardButtonPollType;
use crate::api::types::keyboard_button_request_chat::KeyboardButtonRequestChat;
use crate::api::types::keyboard_button_request_users::KeyboardButtonRequestUsers;
use crate::api::types::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#keyboardbutton
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyboardButton {
    text: String,
    request_users: Option<KeyboardButtonRequestUsers>,
    request_chat: Option<KeyboardButtonRequestChat>,
    request_contact: Option<bool>,
    request_location: Option<bool>,
    request_poll: Option<KeyboardButtonPollType>,
    web_app: Option<WebAppInfo>,
}
