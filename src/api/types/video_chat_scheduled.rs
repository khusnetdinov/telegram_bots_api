use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#videochatscheduled
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VideoChatScheduled {
    start_date: i64,
}
