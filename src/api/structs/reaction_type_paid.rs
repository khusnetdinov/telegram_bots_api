use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#reactiontypepaid>
/// The reaction is paid.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReactionTypePaid {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
