use crate::api::types::location::Location;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequery
/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQuery {
    id: String,
    from: User,
    query: String,
    offset: String,
    chat_type: Option<String>,
    location: Option<Location>,
}
