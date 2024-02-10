use crate::api::enums::input_message_content::InputMessageContent;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultarticle
/// Represents a link to an article or web page.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultArticle {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub title: String,
    pub input_message_content: InputMessageContent,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub url: Option<String>,
    pub hide_url: Option<bool>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<i64>,
    pub thumbnail_height: Option<i64>,
}
