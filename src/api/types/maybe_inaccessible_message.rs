use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#maybeinaccessiblemessage
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MaybeInaccessibleMessage {}
