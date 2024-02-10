use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultlocation
/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the location.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultLocation {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<i64>,
    pub thumbnail_height: Option<i64>,
}
