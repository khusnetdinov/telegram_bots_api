use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#reactioncount
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReactionCount {
    // type: ReactionType,
    total_count: i64,
}
