use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#location>
/// This object represents a point on the map.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
}
