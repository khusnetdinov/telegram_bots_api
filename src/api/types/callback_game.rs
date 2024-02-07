use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#callbackgame
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CallbackGame {}
