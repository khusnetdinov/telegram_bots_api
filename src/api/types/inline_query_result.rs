use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresult
#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResult {}
