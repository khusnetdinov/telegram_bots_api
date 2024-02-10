use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#declinechatjoinrequest
/// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
#[derive(Debug, Serialize)]
struct DeclineChatJoinRequest {
    pub chat_id: ChatUId,
    pub user_id: i64,
}
