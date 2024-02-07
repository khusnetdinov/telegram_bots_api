use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresultcontact
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultContact {
    // type: String,
    id: String,
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumbnail_url: Option<String>,
    thumbnail_width: Option<i64>,
    thumbnail_height: Option<i64>,
}
