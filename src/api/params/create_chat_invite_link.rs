use serde::Serialize;

/// https://core.telegram.org/bots/api#createchatinvitelink
/// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method revokeChatInviteLink. Returns the new invite link as ChatInviteLink object.
#[derive(Debug, Serialize)]
pub struct CreateChatInviteLink {
    chat_id: i64,
    name: Option<String>,
    expire_date: Option<i64>,
    member_limit: Option<i64>,
    creates_join_request: Option<bool>,
}
