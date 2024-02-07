use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatboostsource
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostSource {}
