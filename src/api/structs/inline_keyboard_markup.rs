use crate::api::structs::inline_keyboard_button::InlineKeyboardButton;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inlinekeyboardmarkup>
/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
