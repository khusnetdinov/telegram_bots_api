use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#revokechatinvitelink
/// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as ChatInviteLink object.
#[derive(Debug, Serialize)]
pub struct RevokeChatInviteLink {
    pub chat_id: ChatUId,
    pub invite_link: String,
}
