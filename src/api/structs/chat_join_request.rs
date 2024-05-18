use crate::api::structs::chat::Chat;
use crate::api::structs::chat_invite_link::ChatInviteLink;
use crate::api::structs::user::User;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatjoinrequest>
/// Represents a join request sent to a chat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatJoinRequest {
    pub chat: Chat,
    pub from: User,
    pub user_chat_id: i64,
    pub date: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}
