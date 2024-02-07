use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inputmedia
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputMedia {}
