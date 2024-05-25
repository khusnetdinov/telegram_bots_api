use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#backgroundfillfreeformgradient>
/// The background is a freeform gradient that rotates after every message in the chat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundFillFreeformGradient {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub colors: Vec<u32>,
}
