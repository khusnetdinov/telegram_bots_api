use crate::api::types::chat::Chat;
use crate::api::types::chat_invite_link::ChatInviteLink;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatjoinrequest
/// Represents a join request sent to a chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatJoinRequest {
    chat: Chat,
    from: User,
    user_chat_id: i64,
    date: i64,
    bio: Option<String>,
    invite_link: Option<ChatInviteLink>,
}
