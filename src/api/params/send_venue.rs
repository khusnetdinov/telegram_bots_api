use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendvenue
/// Use this method to send information about a venue. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendVenue {
    chat_id: i64,
    message_thread_id: Option<i64>,
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<ReplyParameters>,
    reply_markup: Option<ReplyMarkup>,
}
