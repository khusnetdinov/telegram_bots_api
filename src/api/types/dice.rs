use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#dice
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Dice {
    emoji: String,
    value: i64,
}
