use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#gamehighscore
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameHighScore {
    position: i64,
    user: User,
    score: i64,
}
