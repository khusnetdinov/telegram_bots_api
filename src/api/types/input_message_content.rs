use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inputmessagecontent
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputMessageContent {}
