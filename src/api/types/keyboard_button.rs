use crate::api::types::keyboard_button_poll_type::KeyboardButtonPollType;
use crate::api::types::keyboard_button_request_chat::KeyboardButtonRequestChat;
use crate::api::types::keyboard_button_request_users::KeyboardButtonRequestUsers;
use crate::api::types::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#keyboardbutton
/// This object represents one button of the reply keyboard. For simple text buttons, String can be used instead of this object to specify the button text. The optional fields web_app, request_users, request_chat, request_contact, request_location, and request_poll are mutually exclusive.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KeyboardButton {
    text: String,
    request_users: Option<KeyboardButtonRequestUsers>,
    request_chat: Option<KeyboardButtonRequestChat>,
    request_contact: Option<bool>,
    request_location: Option<bool>,
    request_poll: Option<KeyboardButtonPollType>,
    web_app: Option<WebAppInfo>,
}
