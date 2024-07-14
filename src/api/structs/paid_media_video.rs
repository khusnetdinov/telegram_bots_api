use crate::api::structs::video::Video;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#paidmediavideo>
/// The paid media is a video.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaidMediaVideo {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub video: Video,
}
