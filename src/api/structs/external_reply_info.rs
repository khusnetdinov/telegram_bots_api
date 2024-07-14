use crate::api::enums::message_origin::MessageOrigin;
use crate::api::structs::animation::Animation;
use crate::api::structs::audio::Audio;
use crate::api::structs::chat::Chat;
use crate::api::structs::contact::Contact;
use crate::api::structs::dice::Dice;
use crate::api::structs::document::Document;
use crate::api::structs::game::Game;
use crate::api::structs::giveaway::Giveaway;
use crate::api::structs::giveaway_winners::GiveawayWinners;
use crate::api::structs::invoice::Invoice;
use crate::api::structs::link_preview_options::LinkPreviewOptions;
use crate::api::structs::location::Location;
use crate::api::structs::paid_media_info::PaidMediaInfo;
use crate::api::structs::photo_size::PhotoSize;
use crate::api::structs::poll::Poll;
use crate::api::structs::sticker::Sticker;
use crate::api::structs::story::Story;
use crate::api::structs::venue::Venue;
use crate::api::structs::video::Video;
use crate::api::structs::video_note::VideoNote;
use crate::api::structs::voice::Voice;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#externalreplyinfo>
/// This object contains information about a message that is being replied to, which may come from another chat or forum topic.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,
}
