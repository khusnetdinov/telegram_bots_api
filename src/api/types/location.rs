use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#location
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    longitude: f64,
    latitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<i64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
}
