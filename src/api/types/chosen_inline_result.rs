use crate::api::types::location::Location;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#choseninlineresult
/// Represents a result of an inline query that was chosen by the user and sent to their chat partner.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChosenInlineResult {
    result_id: String,
    from: User,
    location: Option<Location>,
    inline_message_id: Option<String>,
    query: String,
}
