use crate::api::structs::business_opening_hours_interval::BusinessOpeningHoursInterval;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#businessopeninghours>
/// Describes the opening hours of a business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BusinessOpeningHours {
    pub time_zone_name: String,
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
