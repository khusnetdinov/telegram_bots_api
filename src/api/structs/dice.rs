use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#dice>
/// This object represents an animated emoji that displays a random value.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Dice {
    pub emoji: String,
    pub value: i64,
}
