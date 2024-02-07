use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresultgif
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultGif {
    // type: String,
    id: String,
    gif_url: String,
    gif_width: Option<i64>,
    gif_height: Option<i64>,
    gif_duration: Option<i64>,
    thumbnail_url: String,
    thumbnail_mime_type: Option<String>,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}
