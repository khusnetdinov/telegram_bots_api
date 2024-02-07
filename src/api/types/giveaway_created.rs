use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#giveawaycreated
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCreated {}
