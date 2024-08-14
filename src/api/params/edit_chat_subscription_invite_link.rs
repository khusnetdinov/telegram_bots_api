use crate::api::enums::chat_uid::ChatUId;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#editchatsubscriptioninvitelink
/// Use this method to edit a subscription invite link created by the bot. The bot must have the can_invite_users administrator rights. Returns the edited invite link as a ChatInviteLink object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditChatSubscriptionInviteLink {
    pub chat_id: ChatUId,
    pub invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
