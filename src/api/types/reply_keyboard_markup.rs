use crate::api::types::keyboard_button::KeyboardButton;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#replykeyboardmarkup
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<KeyboardButton>,
    is_persistent: Option<bool>,
    resize_keyboard: Option<bool>,
    one_time_keyboard: Option<bool>,
    input_field_placeholder: Option<String>,
    selective: Option<bool>,
}
