use crate::api::types::keyboard_button::KeyboardButton;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#replykeyboardmarkup
/// This object represents a custom keyboard with reply options (see Introduction to bots for details and examples).
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<KeyboardButton>,
    is_persistent: Option<bool>,
    resize_keyboard: Option<bool>,
    one_time_keyboard: Option<bool>,
    input_field_placeholder: Option<String>,
    selective: Option<bool>,
}
