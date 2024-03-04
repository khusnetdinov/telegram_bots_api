use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#banchatmember
/// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct BanChatMember {
    pub chat_id: ChatUId,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}
