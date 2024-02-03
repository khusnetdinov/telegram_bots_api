use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresultphoto
#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultPhoto {
    // type: String,
    id: String,
    photo_url: String,
    thumbnail_url: String,
    photo_width: Option<i64>,
    photo_height: Option<i64>,
    title: Option<String>,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}
