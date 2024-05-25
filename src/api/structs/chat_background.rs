use crate::api::enums::background_types::BackgroundTypes;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatbackground>
/// This object represents a chat background.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBackground {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: BackgroundTypes,
}
