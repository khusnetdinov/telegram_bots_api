use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#reactiontype
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReactionType {}
