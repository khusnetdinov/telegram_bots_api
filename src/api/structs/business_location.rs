use crate::api::structs::location::Location;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#businesslocation>
/// Contains information about the location of a Telegram Business account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BusinessLocation {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}
