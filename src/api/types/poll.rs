use crate::api::types::message_entity::MessageEntity;
use crate::api::types::poll_option::PollOption;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#poll
/// This object contains information about a poll.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Poll {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: i64,
    pub is_closed: bool,
    pub is_anonymous: bool,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<i64>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<i64>,
    pub close_date: Option<i64>,
}
