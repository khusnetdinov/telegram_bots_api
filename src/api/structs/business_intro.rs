use crate::api::structs::sticker::Sticker;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#businessintro>
/// Contains information about the start page settings of a Telegram Business account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessIntro {
    pub title: Option<String>,
    pub message: Option<String>,
    pub sticker: Option<Sticker>,
}
