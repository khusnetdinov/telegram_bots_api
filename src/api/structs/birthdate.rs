use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#birthdate>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Birthdate {
    pub day: u8,
    pub month: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u8>,
}
