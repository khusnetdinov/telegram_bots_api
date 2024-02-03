use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#messageorigin
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageOrigin {}
