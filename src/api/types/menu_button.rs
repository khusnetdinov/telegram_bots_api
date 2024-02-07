use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#menubutton
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MenuButton {}
