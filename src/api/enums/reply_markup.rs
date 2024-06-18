use crate::api::structs::force_reply::ForceReply;
use crate::api::structs::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::structs::reply_keyboard_markup::ReplyKeyboardMarkup;
use crate::api::structs::reply_keyboard_remove::ReplyKeyboardRemove;
use serde::{Deserialize, Serialize};

/// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}
