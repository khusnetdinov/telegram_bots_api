use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#dice
#[derive(Debug, Serialize, Deserialize)]
pub struct Dice {
    emoji: String,
    value: i64,
}
