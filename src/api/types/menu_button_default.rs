use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#menubuttondefault
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MenuButtonDefault {
    // type: String,
}
