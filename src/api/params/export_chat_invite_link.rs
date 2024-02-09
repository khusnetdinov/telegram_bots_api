use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#exportchatinvitelink
/// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as String on success.
#[derive(Debug, Serialize)]
pub struct ExportChatInviteLink {
    chat_id: ChatUId,
}
