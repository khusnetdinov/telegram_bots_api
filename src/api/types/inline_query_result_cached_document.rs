use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresultcacheddocument
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedDocument {
    // type: String,
    id: String,
    title: String,
    document_file_id: String,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}
