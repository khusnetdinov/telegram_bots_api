use crate::api::types::keyboard_button::KeyboardButton;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#replykeyboardmarkup
/// This object represents a custom keyboard with reply options (see Introduction to bots for details and examples).
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<KeyboardButton>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_persistent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}
