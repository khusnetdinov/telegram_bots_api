use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#editchatinvitelink
/// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a ChatInviteLink object.
#[derive(Debug, Serialize)]
struct EditChatInviteLink {
    pub chat_id: ChatUId,
    pub invite_link: String,
    pub name: Option<String>,
    pub expire_date: Option<i64>,
    pub member_limit: Option<i64>,
    pub creates_join_request: Option<bool>,
}
