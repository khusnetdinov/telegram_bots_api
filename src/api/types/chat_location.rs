use crate::api::types::location::Location;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatlocation
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatLocation {
    location: Location,
    address: String,
}