use crate::api::types::chat::Chat;
use crate::api::types::reaction_type::ReactionType;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#messagereactionupdated
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageReactionUpdated {
    chat: Chat,
    message_id: i64,
    user: Option<User>,
    actor_chat: Option<Chat>,
    date: i64,
    old_reaction: Vec<ReactionType>,
    new_reaction: Vec<ReactionType>,
}
