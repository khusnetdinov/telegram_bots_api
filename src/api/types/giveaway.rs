use crate::api::types::chat::Chat;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#giveaway
/// This object represents a message about a scheduled giveaway.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Giveaway {
    pub chats: Vec<Chat>,
    pub winners_selection_date: i64,
    pub winner_count: i64,
    pub only_new_members: Option<bool>,
    pub has_public_winners: Option<bool>,
    pub prize_description: Option<String>,
    pub country_codes: Option<Vec<String>>,
    pub premium_subscription_month_count: Option<i64>,
}
