use crate::api::types::chat::Chat;
use crate::api::types::chat_invite_link::ChatInviteLink;
use crate::api::types::chat_member::ChatMember;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatmemberupdated
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberUpdated {
    chat: Chat,
    from: User,
    date: i64,
    old_chat_member: ChatMember,
    new_chat_member: ChatMember,
    invite_link: Option<ChatInviteLink>,
    via_chat_folder_invite_link: Option<bool>,
}
