use crate::api::types::inline_query_result_article::InlineQueryResultArticle;
use crate::api::types::inline_query_result_audio::InlineQueryResultAudio;
use crate::api::types::inline_query_result_cached_audio::InlineQueryResultCachedAudio;
use crate::api::types::inline_query_result_cached_document::InlineQueryResultCachedDocument;
use crate::api::types::inline_query_result_cached_gif::InlineQueryResultCachedGif;
use crate::api::types::inline_query_result_cached_mpeg4_gif::InlineQueryResultCachedMpeg4Gif;
use crate::api::types::inline_query_result_cached_photo::InlineQueryResultCachedPhoto;
use crate::api::types::inline_query_result_cached_sticker::InlineQueryResultCachedSticker;
use crate::api::types::inline_query_result_cached_video::InlineQueryResultCachedVideo;
use crate::api::types::inline_query_result_cached_voice::InlineQueryResultCachedVoice;
use crate::api::types::inline_query_result_contact::InlineQueryResultContact;
use crate::api::types::inline_query_result_document::InlineQueryResultDocument;
use crate::api::types::inline_query_result_game::InlineQueryResultGame;
use crate::api::types::inline_query_result_gif::InlineQueryResultGif;
use crate::api::types::inline_query_result_location::InlineQueryResultLocation;
use crate::api::types::inline_query_result_mpeg4_gif::InlineQueryResultMpeg4Gif;
use crate::api::types::inline_query_result_photo::InlineQueryResultPhoto;
use crate::api::types::inline_query_result_venue::InlineQueryResultVenue;
use crate::api::types::inline_query_result_video::InlineQueryResultVideo;
use crate::api::types::inline_query_result_voice::InlineQueryResultVoice;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresult
/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:
/// InlineQueryResultCachedAudio
/// InlineQueryResultCachedDocument
/// InlineQueryResultCachedGif
/// InlineQueryResultCachedMpeg4Gif
/// InlineQueryResultCachedPhoto
/// InlineQueryResultCachedSticker
/// InlineQueryResultCachedVideo
/// InlineQueryResultCachedVoice
/// InlineQueryResultArticle
/// InlineQueryResultAudio
/// InlineQueryResultContact
/// InlineQueryResultGame
/// InlineQueryResultDocument
/// InlineQueryResultGif
/// InlineQueryResultLocation
/// InlineQueryResultMpeg4Gif
/// InlineQueryResultPhoto
/// InlineQueryResultVenue
/// InlineQueryResultVideo
/// InlineQueryResultVoice
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum InlineQueryResult {
    CachedAudio(InlineQueryResultCachedAudio),
    CachedDocument(InlineQueryResultCachedDocument),
    CachedGif(InlineQueryResultCachedGif),
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    CachedPhoto(InlineQueryResultCachedPhoto),
    CachedSticker(InlineQueryResultCachedSticker),
    CachedVideo(InlineQueryResultCachedVideo),
    CachedVoice(InlineQueryResultCachedVoice),
    Article(InlineQueryResultArticle),
    Audio(InlineQueryResultAudio),
    Contact(InlineQueryResultContact),
    Game(InlineQueryResultGame),
    Document(InlineQueryResultDocument),
    Gif(InlineQueryResultGif),
    Location(InlineQueryResultLocation),
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    Photo(InlineQueryResultPhoto),
    Venue(InlineQueryResultVenue),
    Video(InlineQueryResultVideo),
    Voice(InlineQueryResultVoice),
}

impl Default for InlineQueryResult {
    fn default() -> Self {
        Self::CachedAudio(InlineQueryResultCachedAudio {
            ..Default::default()
        })
    }
}
