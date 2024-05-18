use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#videochatended>
/// This object represents a service message about a video chat ended in the chat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatEnded {
    pub duration: i64,
}
