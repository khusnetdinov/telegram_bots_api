use crate::api::structs::background_type_chat_theme::BackgroundTypeChatTheme;
use crate::api::structs::background_type_fill::BackgroundTypeFill;
use crate::api::structs::background_type_pattern::BackgroundTypePattern;
use crate::api::structs::background_type_wallpaper::BackgroundTypeWallpaper;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#backgroundtype>
/// This object describes the type of a background. Currently, it can be one of
/// BackgroundTypeFill
/// BackgroundTypeWallpaper
/// BackgroundTypePattern
/// BackgroundTypeChatTheme

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BackgroundType {
    Fill(BackgroundTypeFill),
    Wallpaper(BackgroundTypeWallpaper),
    Pattern(BackgroundTypePattern),
    ChatTheme(BackgroundTypeChatTheme),
}
