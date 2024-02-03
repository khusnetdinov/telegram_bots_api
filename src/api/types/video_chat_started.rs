use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#videochatstarted
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChatStarted {}
