use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresultdocument
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultDocument {
    // type: String,
    id: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    document_url: String,
    mime_type: String,
    description: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumbnail_url: Option<String>,
    thumbnail_width: Option<i64>,
    thumbnail_height: Option<i64>,
}
