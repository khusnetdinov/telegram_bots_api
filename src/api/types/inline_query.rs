use crate::api::types::location::Location;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequery
/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub query: String,
    pub offset: String,
    pub chat_type: Option<String>,
    pub location: Option<Location>,
}
