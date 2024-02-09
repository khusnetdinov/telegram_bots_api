use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatboostsource
/// This object describes the source of a chat boost.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostSource {}
