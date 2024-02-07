use crate::api::types::location::Location;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#venue
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Venue {
    location: Location,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
}
