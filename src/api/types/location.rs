use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#location
/// This object represents a point on the map.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Location {
    longitude: f64,
    latitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<i64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
}
