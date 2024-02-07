use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresultvideo
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultVideo {}
