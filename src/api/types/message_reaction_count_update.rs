use crate::api::types::chat::Chat;
use crate::api::types::reaction_count::ReactionCount;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#messagereactioncountupdated
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageReactionCountUpdated {
    chat: Chat,
    message_id: i64,
    date: i64,
    reactions: Vec<ReactionCount>,
}
