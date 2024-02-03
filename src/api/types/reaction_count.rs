use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#reactioncount
#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionCount {
    // type: ReactionType,
    total_count: i64,
}
