use serde::Serialize;

/// https://core.telegram.org/bots/api#approvechatjoinrequest
/// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
#[derive(Debug, Serialize)]
pub struct ApproveChatJoinRequest {
    chat_id: i64,
    user_id: i64,
}
