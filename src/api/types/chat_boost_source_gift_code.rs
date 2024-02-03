use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatboostsourcegiftcode
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatBoostSourceGiftCode {
    source: String,
    user: User,
}
