use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::chat_permissions::ChatPermissions;
use serde::Serialize;

/// https://core.telegram.org/bots/api#restrictchatmember
/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass True for all permissions to lift restrictions from a user. Returns True on success.
#[derive(Debug, Serialize)]
pub struct RestrictChatMember {
    pub chat_id: ChatUId,
    pub user_id: i64,
    pub permissions: ChatPermissions,
    pub use_independent_chat_permissions: Option<bool>,
    pub until_date: Option<i64>,
}
