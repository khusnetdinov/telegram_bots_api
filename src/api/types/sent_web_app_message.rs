use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#sentwebappmessage
/// Describes an inline message sent by a Web App on behalf of a user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SentWebAppMessage {
    pub inline_message_id: Option<String>,
}
