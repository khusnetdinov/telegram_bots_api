use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultcontact
/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the contact.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultContact {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<i64>,
    pub thumbnail_height: Option<i64>,
}
