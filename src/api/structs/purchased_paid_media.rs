use crate::api::structs::user::User;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#paidmediapurchased>
/// This object contains information about a paid media purchase.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaidMediaPurchased {
    pub from: User,
    pub paid_media_payload: String,
}
