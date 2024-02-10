use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#gamehighscore
/// This object represents one row of the high scores table for a game.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameHighScore {
    pub position: i64,
    pub user: User,
    pub score: i64,
}
