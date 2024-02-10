use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::chat_permissions::ChatPermissions;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setchatpermissions
/// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members administrator rights. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetChatPermissions {
    pub chat_id: ChatUId,
    pub permissions: ChatPermissions,
    pub use_independent_chat_permissions: Option<bool>,
}
