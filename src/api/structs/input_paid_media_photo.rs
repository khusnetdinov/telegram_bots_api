use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inputpaidmediaphoto>
/// The paid media to send is a photo.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputPaidMediaPhoto {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub media: String,
}
