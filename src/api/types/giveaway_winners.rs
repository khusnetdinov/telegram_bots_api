use crate::api::types::chat::Chat;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#giveawaywinners
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GiveawayWinners {
    chat: Chat,
    giveaway_message_id: i64,
    winners_selection_date: i64,
    winner_count: i64,
    winners: Vec<User>,
    additional_chat_count: Option<i64>,
    premium_subscription_month_count: Option<i64>,
    unclaimed_prize_count: Option<i64>,
    only_new_members: Option<bool>,
    was_refunded: Option<bool>,
    prize_description: Option<String>,
}
