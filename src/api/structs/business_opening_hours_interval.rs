use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#businessopeninghoursinterval>
/// Describes an interval of time during which a business is open.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BusinessOpeningHoursInterval {
    pub opening_minute: u16,
    pub closing_minute: u16,
}
