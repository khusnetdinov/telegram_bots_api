use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresultarticle
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultArticle {
    // type: String,
    id: String,
    title: String,
    input_message_content: InputMessageContent,
    reply_markup: Option<InlineKeyboardMarkup>,
    url: Option<String>,
    hide_url: Option<bool>,
    description: Option<String>,
    thumbnail_url: Option<String>,
    thumbnail_width: Option<i64>,
    thumbnail_height: Option<i64>,
}
