use crate::api::enums::reaction_type::ReactionType;
use crate::api::structs::chat::Chat;
use crate::api::structs::message_id::MessageId;
use crate::api::structs::user::User;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#messagereactionupdated>
/// This object represents a change of a reaction on a message performed by a user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageReactionUpdated {
    pub chat: Chat,
    pub message_id: MessageId,
    pub date: i64,
    pub old_reaction: Vec<ReactionType>,
    pub new_reaction: Vec<ReactionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Chat>,
}
