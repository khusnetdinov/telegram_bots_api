use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatboostsourcepremium
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostSourcePremium {
    source: String,
    user: User,
}
