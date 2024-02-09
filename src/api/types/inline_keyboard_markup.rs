use crate::api::types::inline_keyboard_button::InlineKeyboardButton;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinekeyboardmarkup
/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<InlineKeyboardButton>,
}
