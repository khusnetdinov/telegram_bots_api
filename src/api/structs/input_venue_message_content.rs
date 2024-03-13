use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inputvenuemessagecontent>
/// Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct InputVenueMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}
