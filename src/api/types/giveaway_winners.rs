use crate::api::types::chat::Chat;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#giveawaywinners
/// This object represents a message about the completion of a giveaway with public winners.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GiveawayWinners {
    pub chat: Chat,
    pub giveaway_message_id: i64,
    pub winners_selection_date: i64,
    pub winner_count: i64,
    pub winners: Vec<User>,
    pub additional_chat_count: Option<i64>,
    pub premium_subscription_month_count: Option<i64>,
    pub unclaimed_prize_count: Option<i64>,
    pub only_new_members: Option<bool>,
    pub was_refunded: Option<bool>,
    pub prize_description: Option<String>,
}
