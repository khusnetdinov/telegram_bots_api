use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#giveawaycreated>
/// This object represents a service message about the creation of a scheduled giveaway. Currently holds no information.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCreated {}
