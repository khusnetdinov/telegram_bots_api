use crate::api::enums::chat_uid::ChatUId;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#createchatsubscriptioninvitelink>
/// Use this method to create a subscription invite link for a channel chat. The bot must have the can_invite_users administrator rights. The link can be edited using the method editChatSubscriptionInviteLink or revoked using the method revokeChatInviteLink. Returns the new invite link as a ChatInviteLink object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateChatSubscriptionInviteLink {
    pub chat_id: ChatUId,
    pub subscription_period: i64,
    pub subscription_price: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
