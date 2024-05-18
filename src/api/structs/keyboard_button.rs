use crate::api::structs::keyboard_button_poll_type::KeyboardButtonPollType;
use crate::api::structs::keyboard_button_request_chat::KeyboardButtonRequestChat;
use crate::api::structs::keyboard_button_request_users::KeyboardButtonRequestUsers;
use crate::api::structs::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#keyboardbutton>
/// This object represents one button of the reply keyboard. For simple text buttons, String can be used instead of this object to specify the button text. The optional fields web_app, request_users, request_chat, request_contact, request_location, and request_poll are mutually exclusive.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_users: Option<KeyboardButtonRequestUsers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<KeyboardButtonRequestChat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
}
