use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#location
/// This object represents a point on the map.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
}
