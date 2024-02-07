use crate::api::types::location::Location;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequery
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQuery {
    id: String,
    from: User,
    query: String,
    offset: String,
    chat_type: Option<String>,
    location: Option<Location>,
}
