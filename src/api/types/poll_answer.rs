use crate::api::types::chat::Chat;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#pollanswer
/// This object represents an answer of a user in a non-anonymous poll.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PollAnswer {
    pub poll_id: String,
    pub option_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
