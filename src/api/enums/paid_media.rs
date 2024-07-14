use crate::api::structs::paid_media_photo::PaidMediaPhoto;
use crate::api::structs::paid_media_preview::PaidMediaPreview;
use crate::api::structs::paid_media_video::PaidMediaVideo;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#paidmedia>
/// This object describes paid media. Currently, it can be one of
/// PaidMediaPreview
/// PaidMediaPhoto
/// PaidMediaVideo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

pub enum PaidMedia {
    Preview(PaidMediaPreview),
    Photo(PaidMediaPhoto),
    Video(PaidMediaVideo),
}
