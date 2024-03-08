use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#story>
/// This object represents a message about a forwarded story in the chat. Currently holds no information.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Story {}
