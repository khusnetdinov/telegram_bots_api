use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#usersshared>
/// This object contains information about the users whose identifiers were shared with the bot using a KeyboardButtonRequestUsers button.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UsersShared {
    pub request_id: i64,
    pub user_ids: Vec<i64>,
}
