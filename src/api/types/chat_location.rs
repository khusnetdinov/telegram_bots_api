use crate::api::types::location::Location;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatlocation>
/// Represents a location to which a chat is connected.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatLocation {
    pub location: Location,
    pub address: String,
}
