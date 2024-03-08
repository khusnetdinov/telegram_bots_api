use serde::Serialize;

/// <https://core.telegram.org/bots/api#getuserprofilephotos>
/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
#[derive(Debug, Serialize, Default)]
pub struct GetUserProfilePhotos {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}
