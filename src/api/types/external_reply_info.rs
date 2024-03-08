use crate::api::enums::message_origin::MessageOrigin;
use crate::api::types::animation::Animation;
use crate::api::types::audio::Audio;
use crate::api::types::chat::Chat;
use crate::api::types::contact::Contact;
use crate::api::types::dice::Dice;
use crate::api::types::document::Document;
use crate::api::types::game::Game;
use crate::api::types::giveaway::Giveaway;
use crate::api::types::giveaway_winners::GiveawayWinners;
use crate::api::types::invoice::Invoice;
use crate::api::types::link_preview_options::LinkPreviewOptions;
use crate::api::types::location::Location;
use crate::api::types::photo_size::PhotoSize;
use crate::api::types::poll::Poll;
use crate::api::types::sticker::Sticker;
use crate::api::types::story::Story;
use crate::api::types::venue::Venue;
use crate::api::types::video::Video;
use crate::api::types::video_note::VideoNote;
use crate::api::types::voice::Voice;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#externalreplyinfo>
/// This object contains information about a message that is being replied to, which may come from another chat or forum topic.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExternalReplyInfo {
    pub origin: MessageOrigin,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
}
