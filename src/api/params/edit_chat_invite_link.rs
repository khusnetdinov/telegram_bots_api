use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#editchatinvitelink
/// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a ChatInviteLink object.
#[derive(Debug, Serialize)]
struct EditChatInviteLink {
    chat_id: ChatUId,
    invite_link: String,
    name: Option<String>,
    expire_date: Option<i64>,
    member_limit: Option<i64>,
    creates_join_request: Option<bool>,
}
