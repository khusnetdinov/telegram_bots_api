use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#forcereply
#[derive(Debug, Serialize, Deserialize)]
pub struct ForceReply {
    force_reply: bool,
    input_field_placeholder: Option<String>,
    selective: Option<bool>,
}
