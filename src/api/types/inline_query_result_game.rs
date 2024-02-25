use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultgame
/// Represents a Game.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct InlineQueryResultGame {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
