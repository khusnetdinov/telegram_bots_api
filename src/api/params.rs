use crate::api::enums::ReplyMarkup;
use crate::api::types::{InputFile, LinkPreviewOptions, MessageEntity, ReplyParameters};

use serde::Serialize;

/// https://core.telegram.org/bots/api#getupdates
#[derive(Debug, Serialize)]
pub struct GetUpdateParams {
    pub offset: i64,
    pub limit: i64,
    pub timeout: u64,
}

/// https://core.telegram.org/bots/api#setwebhook
#[derive(Debug, Serialize, Default)]
#[serde_with_macros::skip_serializing_none]
pub struct SetWebhookParams {
    pub url: String,
    pub certificate: Option<InputFile>,
    pub ip_address: Option<String>,
    pub max_connections: Option<u32>,
    pub allowed_updates: Option<Vec<String>>,
    pub drop_pending_updates: Option<bool>,
    pub secret_token: Option<String>,
}

/// https://core.telegram.org/bots/api#deletewebhook
#[derive(Debug, Serialize, Default)]
#[serde_with_macros::skip_serializing_none]
pub struct DeleteWebhookParams {
    pub drop_pending_updates: Option<bool>,
}

/// https://core.telegram.org/bots/api#sendmessage
#[derive(Debug, Serialize, Default)]
#[serde_with_macros::skip_serializing_none]
pub struct SendMessageParams {
    pub chat_id: i64,
    pub message_thread_id: Option<i64>,
    pub text: String,
    pub parse_mode: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

// // https://core.telegram.org/bots/api#forwardmessage
// struct ForwardMessageParams {
//     // chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     // from_chat_id: i64 or String,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     message_id: i64,
// }

// // https://core.telegram.org/bots/api#forwardmessages
// struct ForwardMessagesParams {
//     // chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     // from_chat_id: i64 or String,
//     message_ids: Vec<i64>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
// }

// // https://core.telegram.org/bots/api#copymessage
// struct CopyMessageParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     from_chat_id: i64 or String,
//     message_id: i64,
//     caption: Option<String>,
//     parse_mode: Option<String>,
//     caption_entities: Option<Vec<MessageEntity>>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#copymessages
// struct CopyMessagesParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     from_chat_id: i64 or String,
//     message_ids: Vec<i64>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     remove_caption: Option<bool>,
// }

// // https://core.telegram.org/bots/api#sendphoto
// struct SendPhotoParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     photo: InputFile or String,
//     caption: Option<String>,
//     parse_mode: Option<String>,
//     caption_entities: Option<Vec<MessageEntity>>,
//     has_spoiler: Option<bool>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#senddocument
// struct SendDocumentParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     document: InputFile or String,
//     thumbnail: Option<InputFile or String>,
//     caption: Option<String>,
//     parse_mode: Option<String>,
//     caption_entities: Option<Vec<MessageEntity>>,
//     disable_content_type_detection: Option<bool>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#sendvideo
// struct SendVideoParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     video: InputFile or String,
//     duration: Option<i64>,
//     width: Option<i64>,
//     height: Option<i64>,
//     thumbnail: Option<InputFile or String>,
//     caption: Option<String>,
//     parse_mode: Option<String>,
//     caption_entities: Option<Vec<MessageEntity>>,
//     has_spoiler: Option<bool>,
//     supports_streaming: Option<bool>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#sendanimation
// struct SendAnimationParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     animation: InputFile or String,
//     duration: Option<i64>,
//     width: Option<i64>,
//     height: Option<i64>,
//     thumbnail: Option<InputFile or String>,
//     caption: Option<String>,
//     parse_mode: Option<String>,
//     caption_entities: Option<Vec<MessageEntity>>,
//     has_spoiler: Option<bool>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#sendvoice
// struct SendVoiceParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     voice: InputFile or String,
//     caption: Option<String>,
//     parse_mode: Option<String>,
//     caption_entities: Option<Vec<MessageEntity>>,
//     duration: Option<i64>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#sendvideonote
// struct SendVideoNoteParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     video_note: InputFile or String,
//     duration: Option<i64>,
//     length: Option<i64>,
//     thumbnail: Option<InputFile or String>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#sendmediagroup
// struct SendMediaGroupParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     media: Vec<InputMediaAudio, InputMediaDocument, InputMediaPhoto and InputMediaVideo>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
// }

// // https://core.telegram.org/bots/api#sendlocation
// struct SendLocationParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     latitude: f64,
//     longitude: f64,
//     horizontal_accuracy: Option<f64>,
//     live_period: Option<i64>,
//     heading: Option<i64>,
//     proximity_alert_radius: Option<i64>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#sendvenue
// struct SendVenueParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     latitude: f64,
//     longitude: f64,
//     title: String,
//     address: String,
//     foursquare_id: Option<String>,
//     foursquare_type: Option<String>,
//     google_place_id: Option<String>,
//     google_place_type: Option<String>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#sendcontact
// struct SendContactParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     phone_number: String,
//     first_name: String,
//     last_name: Option<String>,
//     vcard: Option<String>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#sendpoll
// struct SendPollParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     question: String,
//     options: Vec<String>,
//     is_anonymous: Option<bool>,
//     type: Option<String>,
//     allows_multiple_answers: Option<bool>,
//     correct_option_id: Option<i64>,
//     explanation: Option<String>,
//     explanation_parse_mode: Option<String>,
//     explanation_entities: Option<Vec<MessageEntity>>,
//     open_period: Option<i64>,
//     close_date: Option<i64>,
//     is_closed: Option<bool>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#senddice
// struct SendDiceParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     emoji: Option<String>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#setmessagereaction
// struct SetMessageReactionParams {
//     chat_id: i64 or String,
//     message_id: i64,
//     reaction: Option<Vec<ReactionType>>,
//     is_big: Option<bool>,
// }

// // https://core.telegram.org/bots/api#getuserprofilephotos
// struct GetUserProfilePhotosParams {
//     user_id: i64,
//     offset: Option<i64>,
//     limit: Option<i64>,
// }

// // https://core.telegram.org/bots/api#getfile
// struct GetFileParams {
//     file_id: String,
// }

// // https://core.telegram.org/bots/api#banchatmember
// struct BanChatMemberParams {
//     chat_id: i64 or String,
//     user_id: i64,
//     until_date: Option<i64>,
//     revoke_messages: Option<bool>,
// }

// // https://core.telegram.org/bots/api#unbanchatmember
// struct UnbanChatMemberParams {
//     chat_id: i64 or String,
//     user_id: i64,
//     only_if_banned: Option<bool>,
// }

// // https://core.telegram.org/bots/api#restrictchatmember
// struct RestrictChatMemberParams {
//     chat_id: i64 or String,
//     user_id: i64,
//     permissions: ChatPermissions,
//     use_independent_chat_permissions: Option<bool>,
//     until_date: Option<i64>,
// }

// // https://core.telegram.org/bots/api#promotechatmember
// struct PromoteChatMemberParams {
//     chat_id: i64 or String,
//     user_id: i64,
//     is_anonymous: Option<bool>,
//     can_manage_chat: Option<bool>,
//     can_delete_messages: Option<bool>,
//     can_manage_video_chats: Option<bool>,
//     can_restrict_members: Option<bool>,
//     can_promote_members: Option<bool>,
//     can_change_info: Option<bool>,
//     can_invite_users: Option<bool>,
//     can_post_messages: Option<bool>,
//     can_edit_messages: Option<bool>,
//     can_pin_messages: Option<bool>,
//     can_post_stories: Option<bool>,
//     can_edit_stories: Option<bool>,
//     can_delete_stories: Option<bool>,
//     can_manage_topics: Option<bool>,
// }

// // https://core.telegram.org/bots/api#setchatadministratorcustomtitle
// struct SetChatAdministratorCustomTitleParams {
//     chat_id: i64 or String,
//     user_id: i64,
//     custom_title: String,
// }

// // https://core.telegram.org/bots/api#banchatsenderchat
// struct BanChatSenderChatParams {
//     chat_id: i64 or String,
//     sender_chat_id: i64,
// }

// // https://core.telegram.org/bots/api#unbanchatsenderchat
// struct UnbanChatSenderChatParams {
//     chat_id: i64 or String,
//     sender_chat_id: i64,
// }

// // https://core.telegram.org/bots/api#setchatpermissions
// struct SetChatPermissionsParams {
//     chat_id: i64 or String,
//     permissions: ChatPermissions,
//     use_independent_chat_permissions: Option<bool>,
// }

// // https://core.telegram.org/bots/api#exportchatinvitelink
// struct ExportChatInviteLinkParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#createchatinvitelink
// struct CreateChatInviteLinkParams {
//     chat_id: i64 or String,
//     name: Option<String>,
//     expire_date: Option<i64>,
//     member_limit: Option<i64>,
//     creates_join_request: Option<bool>,
// }

// // https://core.telegram.org/bots/api#editchatinvitelink
// struct EditChatInviteLinkParams {
//     chat_id: i64 or String,
//     invite_link: String,
//     name: Option<String>,
//     expire_date: Option<i64>,
//     member_limit: Option<i64>,
//     creates_join_request: Option<bool>,
// }

// // https://core.telegram.org/bots/api#revokechatinvitelink
// struct RevokeChatInviteLinkParams {
//     chat_id: i64 or String,
//     invite_link: String,
// }

// // https://core.telegram.org/bots/api#approvechatjoinrequest
// struct ApproveChatJoinRequestParams {
//     chat_id: i64 or String,
//     user_id: i64,
// }

// // https://core.telegram.org/bots/api#declinechatjoinrequest
// struct DeclineChatJoinRequestParams {
//     chat_id: i64 or String,
//     user_id: i64,
// }

// // https://core.telegram.org/bots/api#setchatphoto
// struct SetChatPhotoParams {
//     chat_id: i64 or String,
//     photo: InputFile,
// }

// // https://core.telegram.org/bots/api#deletechatphoto
// struct DeleteChatPhotoParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#setchattitle
// struct SetChatTitleParams {
//     chat_id: i64 or String,
//     title: String,
// }

// // https://core.telegram.org/bots/api#setchatdescription
// struct SetChatDescriptionParams {
//     chat_id: i64 or String,
//     description: Option<String>,
// }

// // https://core.telegram.org/bots/api#pinchatmessage
// struct PinChatMessageParams {
//     chat_id: i64 or String,
//     message_id: i64,
//     disable_notification: Option<bool>,
// }

// // https://core.telegram.org/bots/api#unpinchatmessage
// struct UnpinChatMessageParams {
//     chat_id: i64 or String,
//     message_id: Option<i64>,
// }

// // https://core.telegram.org/bots/api#unpinallchatmessages
// struct UnpinAllChatMessagesParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#leavechat
// struct LeaveChatParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#getchat
// struct GetChatParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#getchatadministrators
// struct GetChatAdministratorsParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#getchatmembercount
// struct GetChatMemberCountParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#getchatmember
// struct GetChatMemberParams {
//     chat_id: i64 or String,
//     user_id: i64,
// }

// // https://core.telegram.org/bots/api#setchatstickerset
// struct SetChatStickerSetParams {
//     chat_id: i64 or String,
//     sticker_set_name: String,
// }

// // https://core.telegram.org/bots/api#deletechatstickerset
// struct DeleteChatStickerSetParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#getforumtopiciconstickers
// struct GetForumTopicIconStickersParams {
// }

// // https://core.telegram.org/bots/api#createforumtopic
// struct CreateForumTopicParams {
//     chat_id: i64 or String,
//     name: String,
//     icon_color: Option<i64>,
//     icon_custom_emoji_id: Option<String>,
// }

// // https://core.telegram.org/bots/api#editforumtopic
// struct EditForumTopicParams {
//     chat_id: i64 or String,
//     message_thread_id: i64,
//     name: Option<String>,
//     icon_custom_emoji_id: Option<String>,
// }

// // https://core.telegram.org/bots/api#closeforumtopic
// struct CloseForumTopicParams {
//     chat_id: i64 or String,
//     message_thread_id: i64,
// }

// // https://core.telegram.org/bots/api#reopenforumtopic
// struct ReopenForumTopicParams {
//     chat_id: i64 or String,
//     message_thread_id: i64,
// }

// // https://core.telegram.org/bots/api#deleteforumtopic
// struct DeleteForumTopicParams {
//     chat_id: i64 or String,
//     message_thread_id: i64,
// }

// // https://core.telegram.org/bots/api#unpinallforumtopicmessages
// struct UnpinAllForumTopicMessagesParams {
//     chat_id: i64 or String,
//     message_thread_id: i64,
// }

// // https://core.telegram.org/bots/api#editgeneralforumtopic
// struct EditGeneralForumTopicParams {
//     chat_id: i64 or String,
//     name: String,
// }

// // https://core.telegram.org/bots/api#closegeneralforumtopic
// struct CloseGeneralForumTopicParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#reopengeneralforumtopic
// struct ReopenGeneralForumTopicParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#hidegeneralforumtopic
// struct HideGeneralForumTopicParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#unhidegeneralforumtopic
// struct UnhideGeneralForumTopicParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#unpinallgeneralforumtopicmessages
// struct UnpinAllGeneralForumTopicMessagesParams {
//     chat_id: i64 or String,
// }

// // https://core.telegram.org/bots/api#answercallbackquery
// struct AnswerCallbackQueryParams {
// }

// // https://core.telegram.org/bots/api#getuserchatboosts
// struct GetUserChatBoostsParams {
//     chat_id: i64 or String,
//     user_id: i64,
// }

// // https://core.telegram.org/bots/api#setmycommands
// struct SetMyCommandsParams {
//     commands: Vec<BotCommand>,
//     scope: Option<BotCommandScope>,
//     language_code: Option<String>,
// }

// // https://core.telegram.org/bots/api#deletemycommands
// struct DeleteMyCommandsParams {
//     scope: Option<BotCommandScope>,
//     language_code: Option<String>,
// }

// // https://core.telegram.org/bots/api#getmycommands
// struct GetMyCommandsParams {
//     scope: Option<BotCommandScope>,
//     language_code: Option<String>,
// }

// // https://core.telegram.org/bots/api#setmyname
// struct SetMyNameParams {
//     name: Option<String>,
//     language_code: Option<String>,
// }

// // https://core.telegram.org/bots/api#getmyname
// struct GetMyNameParams {
//     language_code: Option<String>,
// }

// // https://core.telegram.org/bots/api#setmydescription
// struct SetMyDescriptionParams {
//     description: Option<String>,
//     language_code: Option<String>,
// }

// // https://core.telegram.org/bots/api#getmydescription
// struct GetMyDescriptionParams {
//     language_code: Option<String>,
// }

// // https://core.telegram.org/bots/api#setmyshortdescription
// struct SetMyShortDescriptionParams {
//     short_description: Option<String>,
//     language_code: Option<String>,
// }

// // https://core.telegram.org/bots/api#getmyshortdescription
// struct GetMyShortDescriptionParams {
//     language_code: Option<String>,
// }

// // https://core.telegram.org/bots/api#setchatmenubutton
// struct SetChatMenuButtonParams {
//     chat_id: Option<i64>,
//     menu_button: Option<MenuButton>,
// }

// // https://core.telegram.org/bots/api#getchatmenubutton
// struct GetChatMenuButtonParams {
//     chat_id: Option<i64>,
// }

// // https://core.telegram.org/bots/api#setmydefaultadministratorrights
// struct SetMyDefaultAdministratorRightsParams {
//     rights: Option<ChatAdministratorRights>,
//     for_channels: Option<bool>,
// }

// // https://core.telegram.org/bots/api#getmydefaultadministratorrights
// struct GetMyDefaultAdministratorRightsParams {
//     for_channels: Option<bool>,
// }

// // https://core.telegram.org/bots/api#editmessagetext
// struct EditMessageTextParams {
//     chat_id: Option<i64 or String>,
//     message_id: Option<i64>,
//     inline_message_id: Option<String>,
//     text: String,
//     parse_mode: Option<String>,
//     entities: Option<Vec<MessageEntity>>,
//     link_preview_options: Option<LinkPreviewOptions>,
//     reply_markup: Option<InlineKeyboardMarkup>,
// }

// // https://core.telegram.org/bots/api#editmessagecaption
// struct EditMessageCaptionParams {
//     chat_id: Option<i64 or String>,
//     message_id: Option<i64>,
//     inline_message_id: Option<String>,
//     caption: Option<String>,
//     parse_mode: Option<String>,
//     caption_entities: Option<Vec<MessageEntity>>,
//     reply_markup: Option<InlineKeyboardMarkup>,
// }

// // https://core.telegram.org/bots/api#editmessagemedia
// struct EditMessageMediaParams {
//     chat_id: Option<i64 or String>,
//     message_id: Option<i64>,
//     inline_message_id: Option<String>,
//     media: InputMedia,
//     reply_markup: Option<InlineKeyboardMarkup>,
// }

// // https://core.telegram.org/bots/api#editmessagelivelocation
// struct EditMessageLiveLocationParams {
//     chat_id: Option<i64 or String>,
//     message_id: Option<i64>,
//     inline_message_id: Option<String>,
//     latitude: f64,
//     longitude: f64,
//     horizontal_accuracy: Option<f64>,
//     heading: Option<i64>,
//     proximity_alert_radius: Option<i64>,
//     reply_markup: Option<InlineKeyboardMarkup>,
// }

// // https://core.telegram.org/bots/api#stopmessagelivelocation
// struct StopMessageLiveLocationParams {
//     chat_id: Option<i64 or String>,
//     message_id: Option<i64>,
//     inline_message_id: Option<String>,
//     reply_markup: Option<InlineKeyboardMarkup>,
// }

// // https://core.telegram.org/bots/api#editmessagereplymarkup
// struct EditMessageReplyMarkupParams {
//     chat_id: Option<i64 or String>,
//     message_id: Option<i64>,
//     inline_message_id: Option<String>,
//     reply_markup: Option<InlineKeyboardMarkup>,
// }

// // https://core.telegram.org/bots/api#stoppoll
// struct StopPollParams {
//     chat_id: i64 or String,
//     message_id: i64,
//     reply_markup: Option<InlineKeyboardMarkup>,
// }

// // https://core.telegram.org/bots/api#deletemessage
// struct DeleteMessageParams {
//     chat_id: i64 or String,
//     message_id: i64,
// }

// // https://core.telegram.org/bots/api#deletemessages
// struct DeleteMessagesParams {
//     chat_id: i64 or String,
//     message_ids: Vec<i64>,
// }

// // https://core.telegram.org/bots/api#sendsticker
// struct SendStickerParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     sticker: InputFile or String,
//     emoji: Option<String>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply>,
// }

// // https://core.telegram.org/bots/api#getstickerset
// struct GetStickerSetParams {
//     name: String,
// }

// // https://core.telegram.org/bots/api#getcustomemojistickers
// struct GetCustomEmojiStickersParams {
//     custom_emoji_ids: Vec<String>,
// }

// // https://core.telegram.org/bots/api#uploadstickerfile
// struct UploadStickerFileParams {
//     user_id: i64,
//     sticker: InputFile,
//     sticker_format: String,
// }

// // https://core.telegram.org/bots/api#createnewstickerset
// struct CreateNewStickerSetParams {
//     user_id: i64,
//     name: String,
//     title: String,
//     stickers: Vec<InputSticker>,
//     sticker_format: String,
//     sticker_type: Option<String>,
//     needs_repainting: Option<bool>,
// }

// // https://core.telegram.org/bots/api#addstickertoset
// struct AddStickerToSetParams {
//     user_id: i64,
//     name: String,
//     sticker: InputSticker,
// }

// // https://core.telegram.org/bots/api#setstickerpositioninset
// struct SetStickerPositionInSetParams {
//     sticker: String,
//     position: i64,
// }

// // https://core.telegram.org/bots/api#deletestickerfromset
// struct DeleteStickerFromSetParams {
//     sticker: String,
// }

// // https://core.telegram.org/bots/api#setstickeremojilist
// struct SetStickerEmojiListParams {
//     sticker: String,
//     emoji_list: Vec<String>,
// }

// // https://core.telegram.org/bots/api#setstickerkeywords
// struct SetStickerKeywordsParams {
//     sticker: String,
//     keywords: Option<Vec<String>>,
// }

// // https://core.telegram.org/bots/api#setstickermaskposition
// struct SetStickerMaskPositionParams {
//     sticker: String,
//     mask_position: Option<MaskPosition>,
// }

// // https://core.telegram.org/bots/api#setstickersettitle
// struct SetStickerSetTitleParams {
//     name: String,
//     title: String,
// }

// // https://core.telegram.org/bots/api#setstickersetthumbnail
// struct SetStickerSetThumbnailParams {
//     name: String,
//     user_id: i64,
//     thumbnail: Option<InputFile or String>,
// }

// // https://core.telegram.org/bots/api#setcustomemojistickersetthumbnail
// struct SetCustomEmojiStickerSetThumbnailParams {
//     name: String,
//     custom_emoji_id: Option<String>,
// }

// // https://core.telegram.org/bots/api#deletestickerset
// struct DeleteStickerSetParams {
//     name: String,
// }

// // https://core.telegram.org/bots/api#answerinlinequery
// struct AnswerInlineQueryParams {
//     inline_query_id: String,
//     results: Vec<InlineQueryResult>,
//     cache_time: Option<i64>,
//     is_personal: Option<bool>,
//     next_offset: Option<String>,
//     button: Option<InlineQueryResultsButton>,
// }

// // https://core.telegram.org/bots/api#answerwebappquery
// struct AnswerWebAppQueryParams {
//     web_app_query_id: String,
//     result: InlineQueryResult,
// }

// // https://core.telegram.org/bots/api#sendinvoice
// struct SendInvoiceParams {
//     chat_id: i64 or String,
//     message_thread_id: Option<i64>,
//     title: String,
//     description: String,
//     payload: String,
//     provider_token: String,
//     currency: String,
//     prices: Vec<LabeledPrice>,
//     max_tip_amount: Option<i64>,
//     suggested_tip_amounts: Option<Vec<i64>>,
//     start_parameter: Option<String>,
//     provider_data: Option<String>,
//     photo_url: Option<String>,
//     photo_size: Option<i64>,
//     photo_width: Option<i64>,
//     photo_height: Option<i64>,
//     need_name: Option<bool>,
//     need_phone_number: Option<bool>,
//     need_email: Option<bool>,
//     need_shipping_address: Option<bool>,
//     send_phone_number_to_provider: Option<bool>,
//     send_email_to_provider: Option<bool>,
//     is_flexible: Option<bool>,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup>,
// }

// // https://core.telegram.org/bots/api#createinvoicelink
// struct CreateInvoiceLinkParams {
//     title: String,
//     description: String,
//     payload: String,
//     provider_token: String,
//     currency: String,
//     prices: Vec<LabeledPrice>,
//     max_tip_amount: Option<i64>,
//     suggested_tip_amounts: Option<Vec<i64>>,
//     provider_data: Option<String>,
//     photo_url: Option<String>,
//     photo_size: Option<i64>,
//     photo_width: Option<i64>,
//     photo_height: Option<i64>,
//     need_name: Option<bool>,
//     need_phone_number: Option<bool>,
//     need_email: Option<bool>,
//     need_shipping_address: Option<bool>,
//     send_phone_number_to_provider: Option<bool>,
//     send_email_to_provider: Option<bool>,
//     is_flexible: Option<bool>,
// }

// // https://core.telegram.org/bots/api#answershippingquery
// struct AnswerShippingQueryParams {
//     shipping_query_id: String,
//     ok: bool,
//     shipping_options: Option<Vec<ShippingOption>>,
//     error_message: Option<String>,
// }

// // https://core.telegram.org/bots/api#answerprecheckoutquery
// struct AnswerPreCheckoutQueryParams {
//     pre_checkout_query_id: String,
//     ok: bool,
//     error_message: Option<String>,
// }

// // https://core.telegram.org/bots/api#setpassportdataerrors
// struct SetPassportDataErrorsParams {
// }

// // https://core.telegram.org/bots/api#sendgame
// struct SendGameParams {
//     chat_id: i64,
//     message_thread_id: Option<i64>,
//     game_short_name: String,
//     disable_notification: Option<bool>,
//     protect_content: Option<bool>,
//     reply_parameters: Option<ReplyParameters>,
//     reply_markup: Option<InlineKeyboardMarkup>,
// }

// // https://core.telegram.org/bots/api#setgamescore
// struct SetGameScoreParams {
//     user_id: i64,
//     score: i64,
//     force: Option<bool>,
//     disable_edit_message: Option<bool>,
//     chat_id: Option<i64>,
//     message_id: Option<i64>,
//     inline_message_id: Option<String>,
// }
