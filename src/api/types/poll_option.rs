use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#polloption
/// This object contains information about one answer option in a poll.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PollOption {
    pub text: String,
    pub voter_count: i64,
}
