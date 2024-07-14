use crate::api::structs::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#paidmediaphoto>
/// The paid media is a photo.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaidMediaPhoto {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub photo: Vec<PhotoSize>,
}
