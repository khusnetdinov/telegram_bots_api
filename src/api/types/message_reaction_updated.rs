use crate::api::enums::reaction_type::ReactionType;
use crate::api::types::chat::Chat;
use crate::api::types::message_id::MessageId;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messagereactionupdated
/// This object represents a change of a reaction on a message performed by a user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageReactionUpdated {
    pub chat: Chat,
    pub message_id: MessageId,
    pub user: Option<User>,
    pub actor_chat: Option<Chat>,
    pub date: i64,
    pub old_reaction: Vec<ReactionType>,
    pub new_reaction: Vec<ReactionType>,
}
