use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputmediaphoto
/// Represents a photo to be sent.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputMediaPhoto {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub media: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub has_spoiler: Option<bool>,
}
