use crate::api::types::force_reply::ForceReply;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::reply_keyboard_markup::ReplyKeyboardMarkup;
use crate::api::types::reply_keyboard_remove::ReplyKeyboardRemove;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}
