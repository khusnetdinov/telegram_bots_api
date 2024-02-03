use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inputlocationmessagecontent
#[derive(Debug, Serialize, Deserialize)]
pub struct InputLocationMessageContent {
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<i64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
}
