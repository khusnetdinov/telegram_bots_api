use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#userprofilephotos
/// This object represent a user's profile pictures.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserProfilePhotos {
    pub total_count: i64,
    pub photos: Vec<Vec<PhotoSize>>,
}
