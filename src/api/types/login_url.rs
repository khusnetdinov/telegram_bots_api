use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#loginurl
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUrl {}
