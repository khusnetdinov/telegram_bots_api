use crate::api::structs::shared_user::SharedUser;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#usersshared>
/// This object contains information about the users whose identifiers were shared with the bot using a KeyboardButtonRequestUsers button.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsersShared {
    pub request_id: i64,
    pub user_ids: Vec<SharedUser>,
}
