use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#forcereply
/// Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice privacy mode.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ForceReply {
    force_reply: bool,
    input_field_placeholder: Option<String>,
    selective: Option<bool>,
}
