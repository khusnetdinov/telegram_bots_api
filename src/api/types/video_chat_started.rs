use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#videochatstarted
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VideoChatStarted {}
