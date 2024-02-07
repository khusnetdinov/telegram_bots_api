use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresultgame
#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
    // type: String,
    id: String,
    game_short_name: String,
    reply_markup: Option<InlineKeyboardMarkup>,
}