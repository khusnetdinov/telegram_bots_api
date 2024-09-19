use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#giveawaycreated>
/// This object represents a service message about the creation of a scheduled giveaway. Currently holds no information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCreated {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
}
