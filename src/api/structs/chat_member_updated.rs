use crate::api::enums::chat_member::ChatMember;
use crate::api::structs::chat::Chat;
use crate::api::structs::chat_invite_link::ChatInviteLink;
use crate::api::structs::user::User;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatmemberupdated>
/// This object represents changes in the status of a chat member.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: i64,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_join_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
}
