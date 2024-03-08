use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inputlocationmessagecontent>
/// Represents the content of a location message to be sent as the result of an inline query.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
}
