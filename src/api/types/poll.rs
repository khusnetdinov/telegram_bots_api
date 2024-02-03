use crate::api::types::message_entity::MessageEntity;
use crate::api::types::poll_option::PollOption;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#poll
#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    id: String,
    question: String,
    options: Vec<PollOption>,
    total_voter_count: i64,
    is_closed: bool,
    is_anonymous: bool,
    // type: String,
    allows_multiple_answers: bool,
    correct_option_id: Option<i64>,
    explanation: Option<String>,
    explanation_entities: Option<Vec<MessageEntity>>,
    open_period: Option<i64>,
    close_date: Option<i64>,
}
