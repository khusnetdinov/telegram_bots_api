use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#polloption
#[derive(Debug, Serialize, Deserialize)]
pub struct PollOption {
    text: String,
    voter_count: i64,
}
