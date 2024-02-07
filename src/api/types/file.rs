use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#file
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct File {}
