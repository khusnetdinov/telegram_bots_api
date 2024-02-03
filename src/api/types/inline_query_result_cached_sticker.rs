use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresultcachedsticker
#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedSticker {
    // type: String,
    id: String,
    sticker_file_id: String,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}
