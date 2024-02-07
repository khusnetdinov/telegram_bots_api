use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botshortdescription
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotShortDescription {
    short_description: String,
}
