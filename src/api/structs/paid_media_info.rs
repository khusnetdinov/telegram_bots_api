use crate::api::enums::paid_media::PaidMedia;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#paidmediainfo>
/// Describes the paid media added to a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaidMediaInfo {
    pub star_count: i64,
    pub paid_media: Vec<PaidMedia>,
}
