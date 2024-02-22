use serde::Serialize;

/// https://core.telegram.org/bots/api#getforumtopiciconstickers
/// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of Sticker objects.
#[derive(Debug, Serialize, Default)]
pub struct GetForumTopicIconStickers {}
