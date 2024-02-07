use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#videochatended
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VideoChatEnded {
    duration: i64,
}
