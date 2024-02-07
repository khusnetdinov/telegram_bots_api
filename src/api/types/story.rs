use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#story
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Story {}
