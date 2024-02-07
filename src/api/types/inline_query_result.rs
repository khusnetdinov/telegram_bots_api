use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresult
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResult {}
