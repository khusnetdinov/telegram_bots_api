use crate::api::enums::input_message_content::InputMessageContent;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultlocation
/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the location.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct InlineQueryResultLocation {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}
