use super::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inputpolloption>
/// This object contains information about one answer option in a poll to send.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct InputPollOption {
    pub text: Option<String>,
    pub text_parse_mode: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
}
