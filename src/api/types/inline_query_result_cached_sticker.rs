use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultcachedsticker
/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the sticker.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedSticker {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub sticker_file_id: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
