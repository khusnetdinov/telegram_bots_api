use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresultlocation
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultLocation {
    // type: String,
    id: String,
    latitude: f64,
    longitude: f64,
    title: String,
    horizontal_accuracy: Option<f64>,
    live_period: Option<i64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumbnail_url: Option<String>,
    thumbnail_width: Option<i64>,
    thumbnail_height: Option<i64>,
}
