use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatmemberleft
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMemberLeft {
    status: String,
    user: User,
}
