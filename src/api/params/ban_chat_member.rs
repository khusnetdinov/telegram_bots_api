use serde::Serialize;

/// https://core.telegram.org/bots/api#banchatmember
/// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Debug, Serialize)]
pub struct BanChatMember {
    chat_id: i64,
    user_id: i64,
    until_date: Option<i64>,
    revoke_messages: Option<bool>,
}
