use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#loginurl
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LoginUrl {}
