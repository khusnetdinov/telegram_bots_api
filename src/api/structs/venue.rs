use crate::api::structs::location::Location;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#venue>
/// This object represents a venue.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Venue {
    pub location: Location,
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
