use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatmember
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMember {}
