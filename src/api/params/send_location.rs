use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendlocation
/// Use this method to send point on the map. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendLocation {
    pub chat_id: ChatUId,
    pub message_thread_id: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
