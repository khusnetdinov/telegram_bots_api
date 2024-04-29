use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#businessopeninghoursinterval>
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct BusinessOpeningHoursInterval {
    pub opening_minute: u16,
    pub closing_minute: u16,
}
