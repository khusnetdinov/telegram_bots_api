use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#sentwebappmessage
#[derive(Debug, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    inline_message_id: Option<String>,
}
