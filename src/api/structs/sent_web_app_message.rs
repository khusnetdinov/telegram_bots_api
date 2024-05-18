use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#sentwebappmessage>
/// Describes an inline message sent by a Web App on behalf of a user.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SentWebAppMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
