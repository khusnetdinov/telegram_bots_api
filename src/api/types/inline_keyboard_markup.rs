use crate::api::types::inline_keyboard_button::InlineKeyboardButton;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinekeyboardmarkup
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<InlineKeyboardButton>,
}
