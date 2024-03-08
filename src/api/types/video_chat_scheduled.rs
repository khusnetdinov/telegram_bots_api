use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#videochatscheduled>
/// This object represents a service message about a video chat scheduled in the chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VideoChatScheduled {
    pub start_date: i64,
}
