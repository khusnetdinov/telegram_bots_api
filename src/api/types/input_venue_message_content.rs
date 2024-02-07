use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inputvenuemessagecontent
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputVenueMessageContent {
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
}
