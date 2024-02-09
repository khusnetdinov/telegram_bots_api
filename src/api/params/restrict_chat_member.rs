use crate::api::types::chat_permissions::ChatPermissions;
use serde::Serialize;

/// https://core.telegram.org/bots/api#restrictchatmember
/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass True for all permissions to lift restrictions from a user. Returns True on success.
#[derive(Debug, Serialize)]
pub struct RestrictChatMember {
    chat_id: i64,
    user_id: i64,
    permissions: ChatPermissions,
    use_independent_chat_permissions: Option<bool>,
    until_date: Option<i64>,
}
