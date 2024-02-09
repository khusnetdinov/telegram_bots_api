use serde::Serialize;

/// https://core.telegram.org/bots/api#getuserprofilephotos
/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
#[derive(Debug, Serialize)]
pub struct GetUserProfilePhotos {
    user_id: i64,
    offset: Option<i64>,
    limit: Option<i64>,
}
