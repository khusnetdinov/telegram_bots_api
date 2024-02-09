use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputlocationmessagecontent
/// Represents the content of a location message to be sent as the result of an inline query.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputLocationMessageContent {
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<i64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
}
