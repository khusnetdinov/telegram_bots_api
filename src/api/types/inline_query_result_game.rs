use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultgame
/// Represents a Game.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultGame {
    // type: String,
    id: String,
    game_short_name: String,
    reply_markup: Option<InlineKeyboardMarkup>,
}
