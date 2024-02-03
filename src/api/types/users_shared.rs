use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#usersshared
#[derive(Debug, Serialize, Deserialize)]
pub struct UsersShared {
    request_id: i64,
    user_ids: Vec<i64>,
}
