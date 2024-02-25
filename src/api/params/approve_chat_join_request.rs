use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#approvechatjoinrequest
/// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct ApproveChatJoinRequest {
    pub chat_id: ChatUId,
    pub user_id: i64,
}
