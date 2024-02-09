use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendlocation
/// Use this method to send point on the map. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendLocation {
    chat_id: i64,
    message_thread_id: Option<i64>,
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<i64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<ReplyParameters>,
    reply_markup: Option<ReplyMarkup>,
}
