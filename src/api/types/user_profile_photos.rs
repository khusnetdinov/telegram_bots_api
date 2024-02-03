use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#userprofilephotos
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    total_count: i64,
    photos: Vec<PhotoSize>,
}
