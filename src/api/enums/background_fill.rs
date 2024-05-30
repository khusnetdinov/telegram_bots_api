use crate::api::structs::background_fill_freeform_gradient::BackgroundFillFreeformGradient;
use crate::api::structs::background_fill_gradient::BackgroundFillGradient;
use crate::api::structs::background_fill_solid::BackgroundFillSolid;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#backgroundfill>
/// This object describes the way a background is filled based on the selected colors. Currently, it can be one of
/// BackgroundFillSolid
/// BackgroundFillGradient
/// BackgroundFillFreeformGradient
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BackgroundFill {
    Solid(BackgroundFillSolid),
    Gradient(BackgroundFillGradient),
    FreeformGradient(BackgroundFillFreeformGradient),
}
