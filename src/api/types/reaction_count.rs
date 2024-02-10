use crate::api::enums::reaction_type::ReactionType;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#reactioncount
/// Represents a reaction added to a message along with the number of times it was added.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReactionCount {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: ReactionType,
    pub total_count: i64,
}
