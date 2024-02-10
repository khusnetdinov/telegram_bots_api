use crate::api::types::chat::Chat;
use crate::api::types::message_id::MessageId;
use crate::api::types::reaction_type::ReactionType;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messagereactionupdated
/// This object represents a change of a reaction on a message performed by a user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageReactionUpdated {
    chat: Chat,
    message_id: MessageId,
    user: Option<User>,
    actor_chat: Option<Chat>,
    date: i64,
    old_reaction: Vec<ReactionType>,
    new_reaction: Vec<ReactionType>,
}
