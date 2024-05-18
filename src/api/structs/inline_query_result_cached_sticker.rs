use crate::api::enums::input_message_content::InputMessageContent;
use crate::api::structs::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inlinequeryresultcachedsticker>
/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the sticker.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InlineQueryResultCachedSticker {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub sticker_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
