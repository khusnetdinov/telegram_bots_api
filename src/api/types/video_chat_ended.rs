use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#videochatended
/// This object represents a service message about a video chat ended in the chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VideoChatEnded {
    duration: i64,
}
