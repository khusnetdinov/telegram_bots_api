use crate::api::types::keyboard_button::KeyboardButton;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#replykeyboardmarkup
/// This object represents a custom keyboard with reply options (see Introduction to bots for details and examples).
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<KeyboardButton>,
    pub is_persistent: Option<bool>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}
