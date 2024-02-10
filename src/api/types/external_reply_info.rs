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

/// https://core.telegram.org/bots/api#externalreplyinfo
/// This object contains information about a message that is being replied to, which may come from another chat or forum topic.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExternalReplyInfo {
    origin: MessageOrigin,
    chat: Option<Chat>,
    message_id: Option<i64>,
    link_preview_options: Option<LinkPreviewOptions>,
    animation: Option<Animation>,
    audio: Option<Audio>,
    document: Option<Document>,
    photo: Option<Vec<PhotoSize>>,
    sticker: Option<Sticker>,
    story: Option<Story>,
    video: Option<Video>,
    video_note: Option<VideoNote>,
    voice: Option<Voice>,
    has_media_spoiler: Option<bool>,
    contact: Option<Contact>,
    dice: Option<Dice>,
    game: Option<Game>,
    giveaway: Option<Giveaway>,
    giveaway_winners: Option<GiveawayWinners>,
    invoice: Option<Invoice>,
    location: Option<Location>,
    poll: Option<Poll>,
    venue: Option<Venue>,
}
