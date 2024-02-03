use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#videochatended
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChatEnded {
    duration: i64,
}
