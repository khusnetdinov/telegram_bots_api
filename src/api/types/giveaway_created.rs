use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#giveawaycreated
#[derive(Debug, Serialize, Deserialize)]
pub struct GiveawayCreated {}
