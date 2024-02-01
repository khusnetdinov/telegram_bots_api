pub mod message;
pub mod message_id;
pub mod update;
pub mod user;
pub mod webhook_info;

use crate::api::types::message::Message;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chat
#[derive(Debug, Serialize, Deserialize, Default)]
struct Chat {
    id: i64,
    // type: String,
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    is_forum: Option<bool>,
    photo: Option<ChatPhoto>,
    active_usernames: Option<Vec<String>>,
    available_reactions: Option<Vec<ReactionType>>,
    accent_color_id: Option<i64>,
    background_custom_emoji_id: Option<String>,
    profile_accent_color_id: Option<i64>,
    profile_background_custom_emoji_id: Option<String>,
    emoji_status_custom_emoji_id: Option<String>,
    emoji_status_expiration_date: Option<i64>,
    bio: Option<String>,
    has_private_forwards: Option<bool>,
    has_restricted_voice_and_video_messages: Option<bool>,
    join_to_send_messages: Option<bool>,
    join_by_request: Option<bool>,
    description: Option<String>,
    invite_link: Option<String>,
    pinned_message: Option<Box<Message>>,
    permissions: Option<ChatPermissions>,
    slow_mode_delay: Option<i64>,
    message_auto_delete_time: Option<i64>,
    has_aggressive_anti_spam_enabled: Option<bool>,
    has_hidden_members: Option<bool>,
    has_protected_content: Option<bool>,
    has_visible_history: Option<bool>,
    sticker_set_name: Option<String>,
    can_set_sticker_set: Option<bool>,
    linked_chat_id: Option<i64>,
    location: Option<ChatLocation>,
}

// https://core.telegram.org/bots/api#inaccessiblemessage
#[derive(Debug, Serialize, Deserialize)]
struct InaccessibleMessage {
    chat: Chat,
    message_id: i64,
    date: i64,
}

// https://core.telegram.org/bots/api#maybeinaccessiblemessage
#[derive(Debug, Serialize, Deserialize)]
struct MaybeInaccessibleMessage {}

// https://core.telegram.org/bots/api#messageentity
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEntity {
    // type: String,
    offset: i64,
    length: i64,
    url: Option<String>,
    user: Option<User>,
    language: Option<String>,
    custom_emoji_id: Option<String>,
}

// https://core.telegram.org/bots/api#textquote
#[derive(Debug, Serialize, Deserialize)]
struct TextQuote {
    text: String,
    entities: Option<Vec<MessageEntity>>,
    position: i64,
    is_manual: Option<bool>,
}

// https://core.telegram.org/bots/api#externalreplyinfo
#[derive(Debug, Serialize, Deserialize)]
struct ExternalReplyInfo {
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

/// https://core.telegram.org/bots/api#replyparameters
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyParameters {
    message_id: i64,
    chat_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    quote: Option<String>,
    quote_parse_mode: Option<String>,
    quote_entities: Option<Vec<MessageEntity>>,
    quote_position: Option<i64>,
}

// https://core.telegram.org/bots/api#messageorigin
#[derive(Debug, Serialize, Deserialize)]
struct MessageOrigin {}

// https://core.telegram.org/bots/api#messageoriginuser
#[derive(Debug, Serialize, Deserialize)]
struct MessageOriginUser {
    // type: String,
    date: i64,
    sender_user: User,
}

// https://core.telegram.org/bots/api#messageoriginhiddenuser
#[derive(Debug, Serialize, Deserialize)]
struct MessageOriginHiddenUser {
    // type: String,
    date: i64,
    sender_user_name: String,
}

// https://core.telegram.org/bots/api#messageoriginchat
#[derive(Debug, Serialize, Deserialize)]
struct MessageOriginChat {
    // type: String,
    date: i64,
    sender_chat: Chat,
    author_signature: Option<String>,
}

// https://core.telegram.org/bots/api#messageoriginchannel
#[derive(Debug, Serialize, Deserialize)]
struct MessageOriginChannel {
    // type: String,
    date: i64,
    chat: Chat,
    message_id: i64,
    author_signature: Option<String>,
}

// https://core.telegram.org/bots/api#photosize
#[derive(Debug, Serialize, Deserialize)]
struct PhotoSize {
    file_id: String,
    file_unique_id: String,
    width: i64,
    height: i64,
    file_size: Option<i64>,
}

// https://core.telegram.org/bots/api#animation
#[derive(Debug, Serialize, Deserialize)]
struct Animation {
    file_id: String,
    file_unique_id: String,
    width: i64,
    height: i64,
    duration: i64,
    thumbnail: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}

// https://core.telegram.org/bots/api#audio
#[derive(Debug, Serialize, Deserialize)]
struct Audio {
    file_id: String,
    file_unique_id: String,
    duration: i64,
    performer: Option<String>,
    title: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
    thumbnail: Option<PhotoSize>,
}

// https://core.telegram.org/bots/api#document
#[derive(Debug, Serialize, Deserialize)]
struct Document {
    file_id: String,
    file_unique_id: String,
    thumbnail: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}

// https://core.telegram.org/bots/api#story
#[derive(Debug, Serialize, Deserialize)]
struct Story {}

// https://core.telegram.org/bots/api#video
#[derive(Debug, Serialize, Deserialize)]
struct Video {
    file_id: String,
    file_unique_id: String,
    width: i64,
    height: i64,
    duration: i64,
    thumbnail: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}

// https://core.telegram.org/bots/api#videonote
#[derive(Debug, Serialize, Deserialize)]
struct VideoNote {
    file_id: String,
    file_unique_id: String,
    length: i64,
    duration: i64,
    thumbnail: Option<PhotoSize>,
    file_size: Option<i64>,
}

// https://core.telegram.org/bots/api#voice
#[derive(Debug, Serialize, Deserialize)]
struct Voice {
    file_id: String,
    file_unique_id: String,
    duration: i64,
    mime_type: Option<String>,
    file_size: Option<i64>,
}

// https://core.telegram.org/bots/api#contact
#[derive(Debug, Serialize, Deserialize)]
struct Contact {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    user_id: Option<i64>,
    vcard: Option<String>,
}

// https://core.telegram.org/bots/api#dice
#[derive(Debug, Serialize, Deserialize)]
struct Dice {
    emoji: String,
    value: i64,
}

// https://core.telegram.org/bots/api#polloption
#[derive(Debug, Serialize, Deserialize)]
struct PollOption {
    text: String,
    voter_count: i64,
}

// https://core.telegram.org/bots/api#pollanswer
#[derive(Debug, Serialize, Deserialize)]
struct PollAnswer {
    poll_id: String,
    voter_chat: Option<Chat>,
    user: Option<User>,
    option_ids: Vec<i64>,
}

// https://core.telegram.org/bots/api#poll
#[derive(Debug, Serialize, Deserialize)]
struct Poll {
    id: String,
    question: String,
    options: Vec<PollOption>,
    total_voter_count: i64,
    is_closed: bool,
    is_anonymous: bool,
    // type: String,
    allows_multiple_answers: bool,
    correct_option_id: Option<i64>,
    explanation: Option<String>,
    explanation_entities: Option<Vec<MessageEntity>>,
    open_period: Option<i64>,
    close_date: Option<i64>,
}

// https://core.telegram.org/bots/api#location
#[derive(Debug, Serialize, Deserialize)]
struct Location {
    longitude: f64,
    latitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<i64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
}

// https://core.telegram.org/bots/api#venue
#[derive(Debug, Serialize, Deserialize)]
struct Venue {
    location: Location,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
}

// https://core.telegram.org/bots/api#webappdata
#[derive(Debug, Serialize, Deserialize)]
struct WebAppData {
    data: String,
    button_text: String,
}

// https://core.telegram.org/bots/api#proximityalerttriggered
#[derive(Debug, Serialize, Deserialize)]
struct ProximityAlertTriggered {
    traveler: User,
    watcher: User,
    distance: i64,
}

// https://core.telegram.org/bots/api#messageautodeletetimerchanged
#[derive(Debug, Serialize, Deserialize)]
struct MessageAutoDeleteTimerChanged {
    message_auto_delete_time: i64,
}

// https://core.telegram.org/bots/api#forumtopiccreated
#[derive(Debug, Serialize, Deserialize)]
struct ForumTopicCreated {
    name: String,
    icon_color: i64,
    icon_custom_emoji_id: Option<String>,
}

// https://core.telegram.org/bots/api#forumtopicclosed
#[derive(Debug, Serialize, Deserialize)]
struct ForumTopicClosed {}

// https://core.telegram.org/bots/api#forumtopicedited
#[derive(Debug, Serialize, Deserialize)]
struct ForumTopicEdited {
    name: Option<String>,
    icon_custom_emoji_id: Option<String>,
}

// https://core.telegram.org/bots/api#forumtopicreopened
#[derive(Debug, Serialize, Deserialize)]
struct ForumTopicReopened {}

// https://core.telegram.org/bots/api#generalforumtopichidden
#[derive(Debug, Serialize, Deserialize)]
struct GeneralForumTopicHidden {}

// https://core.telegram.org/bots/api#generalforumtopicunhidden
#[derive(Debug, Serialize, Deserialize)]
struct GeneralForumTopicUnhidden {}

// https://core.telegram.org/bots/api#usersshared
#[derive(Debug, Serialize, Deserialize)]
struct UsersShared {
    request_id: i64,
    user_ids: Vec<i64>,
}

// https://core.telegram.org/bots/api#chatshared
#[derive(Debug, Serialize, Deserialize)]
struct ChatShared {
    request_id: i64,
    chat_id: i64,
}

// https://core.telegram.org/bots/api#writeaccessallowed
#[derive(Debug, Serialize, Deserialize)]
struct WriteAccessAllowed {
    from_request: Option<bool>,
    web_app_name: Option<String>,
    from_attachment_menu: Option<bool>,
}

// https://core.telegram.org/bots/api#videochatscheduled
#[derive(Debug, Serialize, Deserialize)]
struct VideoChatScheduled {
    start_date: i64,
}

// https://core.telegram.org/bots/api#videochatstarted
#[derive(Debug, Serialize, Deserialize)]
struct VideoChatStarted {}

// https://core.telegram.org/bots/api#videochatended
#[derive(Debug, Serialize, Deserialize)]
struct VideoChatEnded {
    duration: i64,
}

// https://core.telegram.org/bots/api#videochatparticipantsinvited
#[derive(Debug, Serialize, Deserialize)]
struct VideoChatParticipantsInvited {
    users: Vec<User>,
}

// https://core.telegram.org/bots/api#giveawaycreated
#[derive(Debug, Serialize, Deserialize)]
struct GiveawayCreated {}

// https://core.telegram.org/bots/api#giveaway
#[derive(Debug, Serialize, Deserialize)]
struct Giveaway {
    chats: Vec<Chat>,
    winners_selection_date: i64,
    winner_count: i64,
    only_new_members: Option<bool>,
    has_public_winners: Option<bool>,
    prize_description: Option<String>,
    country_codes: Option<Vec<String>>,
    premium_subscription_month_count: Option<i64>,
}

// https://core.telegram.org/bots/api#giveawaywinners
#[derive(Debug, Serialize, Deserialize)]
struct GiveawayWinners {
    chat: Chat,
    giveaway_message_id: i64,
    winners_selection_date: i64,
    winner_count: i64,
    winners: Vec<User>,
    additional_chat_count: Option<i64>,
    premium_subscription_month_count: Option<i64>,
    unclaimed_prize_count: Option<i64>,
    only_new_members: Option<bool>,
    was_refunded: Option<bool>,
    prize_description: Option<String>,
}

// https://core.telegram.org/bots/api#giveawaycompleted
#[derive(Debug, Serialize, Deserialize)]
struct GiveawayCompleted {
    winner_count: i64,
    unclaimed_prize_count: Option<i64>,
    giveaway_completed: Option<Box<GiveawayCompleted>>,
}

/// https://core.telegram.org/bots/api#linkpreviewoptions
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
    is_disabled: Option<bool>,
    url: Option<String>,
    prefer_small_media: Option<bool>,
    prefer_large_media: Option<bool>,
    show_above_text: Option<bool>,
}

// https://core.telegram.org/bots/api#userprofilephotos
#[derive(Debug, Serialize, Deserialize)]
struct UserProfilePhotos {
    total_count: i64,
    photos: Vec<PhotoSize>,
}

// https://core.telegram.org/bots/api#file
#[derive(Debug, Serialize, Deserialize)]
struct File {}

// https://core.telegram.org/bots/api#webappinfo
#[derive(Debug, Serialize, Deserialize)]
struct WebAppInfo {
    url: String,
}

// https://core.telegram.org/bots/api#replykeyboardmarkup
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<KeyboardButton>,
    is_persistent: Option<bool>,
    resize_keyboard: Option<bool>,
    one_time_keyboard: Option<bool>,
    input_field_placeholder: Option<String>,
    selective: Option<bool>,
}

// https://core.telegram.org/bots/api#keyboardbutton
#[derive(Debug, Serialize, Deserialize)]
struct KeyboardButton {
    text: String,
    request_users: Option<KeyboardButtonRequestUsers>,
    request_chat: Option<KeyboardButtonRequestChat>,
    request_contact: Option<bool>,
    request_location: Option<bool>,
    request_poll: Option<KeyboardButtonPollType>,
    web_app: Option<WebAppInfo>,
}

// https://core.telegram.org/bots/api#keyboardbuttonrequestusers
#[derive(Debug, Serialize, Deserialize)]
struct KeyboardButtonRequestUsers {
    request_id: i64,
    user_is_bot: Option<bool>,
    user_is_premium: Option<bool>,
    max_quantity: Option<i64>,
}

// https://core.telegram.org/bots/api#keyboardbuttonrequestchat
#[derive(Debug, Serialize, Deserialize)]
struct KeyboardButtonRequestChat {
    request_id: i64,
    chat_is_channel: bool,
    chat_is_forum: Option<bool>,
    chat_has_username: Option<bool>,
    chat_is_created: Option<bool>,
    user_administrator_rights: Option<ChatAdministratorRights>,
    bot_administrator_rights: Option<ChatAdministratorRights>,
    bot_is_member: Option<bool>,
}

// https://core.telegram.org/bots/api#keyboardbuttonpolltype
#[derive(Debug, Serialize, Deserialize)]
struct KeyboardButtonPollType {
    // type: Option<String>,
}

// https://core.telegram.org/bots/api#replykeyboardremove
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    selective: Option<bool>,
}

// https://core.telegram.org/bots/api#inlinekeyboardmarkup
#[derive(Debug, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<InlineKeyboardButton>,
}

// https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, Serialize, Deserialize)]
struct InlineKeyboardButton {
    text: String,
    url: Option<String>,
    callback_data: Option<String>,
    web_app: Option<WebAppInfo>,
    login_url: Option<LoginUrl>,
    switch_inline_query: Option<String>,
    switch_inline_query_current_chat: Option<String>,
    switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    callback_game: Option<CallbackGame>,
    pay: Option<bool>,
}

// https://core.telegram.org/bots/api#loginurl
#[derive(Debug, Serialize, Deserialize)]
struct LoginUrl {}

// https://core.telegram.org/bots/api#switchinlinequerychosenchat
#[derive(Debug, Serialize, Deserialize)]
struct SwitchInlineQueryChosenChat {
    query: Option<String>,
    allow_user_chats: Option<bool>,
    allow_bot_chats: Option<bool>,
    allow_group_chats: Option<bool>,
    allow_channel_chats: Option<bool>,
}

// https://core.telegram.org/bots/api#callbackquery
#[derive(Debug, Serialize, Deserialize)]
struct CallbackQuery {
    id: String,
    from: User,
    message: Option<MaybeInaccessibleMessage>,
    inline_message_id: Option<String>,
    chat_instance: String,
    data: Option<String>,
    game_short_name: Option<String>,
}

// https://core.telegram.org/bots/api#forcereply
#[derive(Debug, Serialize, Deserialize)]
pub struct ForceReply {
    force_reply: bool,
    input_field_placeholder: Option<String>,
    selective: Option<bool>,
}

// https://core.telegram.org/bots/api#chatphoto
#[derive(Debug, Serialize, Deserialize)]
struct ChatPhoto {
    small_file_id: String,
    small_file_unique_id: String,
    big_file_id: String,
    big_file_unique_id: String,
}

// https://core.telegram.org/bots/api#chatinvitelink
#[derive(Debug, Serialize, Deserialize)]
struct ChatInviteLink {
    invite_link: String,
    creator: User,
    creates_join_request: bool,
    is_primary: bool,
    is_revoked: bool,
    name: Option<String>,
    expire_date: Option<i64>,
    member_limit: Option<i64>,
    pending_join_request_count: Option<i64>,
}

// https://core.telegram.org/bots/api#chatadministratorrights
#[derive(Debug, Serialize, Deserialize)]
struct ChatAdministratorRights {
    is_anonymous: bool,
    can_manage_chat: bool,
    can_delete_messages: bool,
    can_manage_video_chats: bool,
    can_restrict_members: bool,
    can_promote_members: bool,
    can_change_info: bool,
    can_invite_users: bool,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_pin_messages: Option<bool>,
    can_post_stories: Option<bool>,
    can_edit_stories: Option<bool>,
    can_delete_stories: Option<bool>,
    can_manage_topics: Option<bool>,
}

// https://core.telegram.org/bots/api#chatmemberupdated
#[derive(Debug, Serialize, Deserialize)]
struct ChatMemberUpdated {
    chat: Chat,
    from: User,
    date: i64,
    old_chat_member: ChatMember,
    new_chat_member: ChatMember,
    invite_link: Option<ChatInviteLink>,
    via_chat_folder_invite_link: Option<bool>,
}

// https://core.telegram.org/bots/api#chatmember
#[derive(Debug, Serialize, Deserialize)]
struct ChatMember {}

// https://core.telegram.org/bots/api#chatmemberowner
#[derive(Debug, Serialize, Deserialize)]
struct ChatMemberOwner {
    status: String,
    user: User,
    is_anonymous: bool,
    custom_title: Option<String>,
}

// https://core.telegram.org/bots/api#chatmemberadministrator
#[derive(Debug, Serialize, Deserialize)]
struct ChatMemberAdministrator {
    status: String,
    user: User,
    can_be_edited: bool,
    is_anonymous: bool,
    can_manage_chat: bool,
    can_delete_messages: bool,
    can_manage_video_chats: bool,
    can_restrict_members: bool,
    can_promote_members: bool,
    can_change_info: bool,
    can_invite_users: bool,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_pin_messages: Option<bool>,
    can_post_stories: Option<bool>,
    can_edit_stories: Option<bool>,
    can_delete_stories: Option<bool>,
    can_manage_topics: Option<bool>,
    custom_title: Option<String>,
}

// https://core.telegram.org/bots/api#chatmembermember
#[derive(Debug, Serialize, Deserialize)]
struct ChatMemberMember {
    status: String,
    user: User,
}

// https://core.telegram.org/bots/api#chatmemberrestricted
#[derive(Debug, Serialize, Deserialize)]
struct ChatMemberRestricted {
    status: String,
    user: User,
    is_member: bool,
    can_send_messages: bool,
    can_send_audios: bool,
    can_send_documents: bool,
    can_send_photos: bool,
    can_send_videos: bool,
    can_send_video_notes: bool,
    can_send_voice_notes: bool,
    can_send_polls: bool,
    can_send_other_messages: bool,
    can_add_web_page_previews: bool,
    can_change_info: bool,
    can_invite_users: bool,
    can_pin_messages: bool,
    can_manage_topics: bool,
    until_date: i64,
}

// https://core.telegram.org/bots/api#chatmemberleft
#[derive(Debug, Serialize, Deserialize)]
struct ChatMemberLeft {
    status: String,
    user: User,
}

// https://core.telegram.org/bots/api#chatmemberbanned
#[derive(Debug, Serialize, Deserialize)]
struct ChatMemberBanned {
    status: String,
    user: User,
    until_date: i64,
}

// https://core.telegram.org/bots/api#chatjoinrequest
#[derive(Debug, Serialize, Deserialize)]
struct ChatJoinRequest {
    chat: Chat,
    from: User,
    user_chat_id: i64,
    date: i64,
    bio: Option<String>,
    invite_link: Option<ChatInviteLink>,
}

// https://core.telegram.org/bots/api#chatpermissions
#[derive(Debug, Serialize, Deserialize)]
struct ChatPermissions {
    can_send_messages: Option<bool>,
    can_send_audios: Option<bool>,
    can_send_documents: Option<bool>,
    can_send_photos: Option<bool>,
    can_send_videos: Option<bool>,
    can_send_video_notes: Option<bool>,
    can_send_voice_notes: Option<bool>,
    can_send_polls: Option<bool>,
    can_send_other_messages: Option<bool>,
    can_add_web_page_previews: Option<bool>,
    can_change_info: Option<bool>,
    can_invite_users: Option<bool>,
    can_pin_messages: Option<bool>,
    can_manage_topics: Option<bool>,
}

// https://core.telegram.org/bots/api#chatlocation
#[derive(Debug, Serialize, Deserialize)]
struct ChatLocation {
    location: Location,
    address: String,
}

// https://core.telegram.org/bots/api#reactiontype
#[derive(Debug, Serialize, Deserialize)]
struct ReactionType {}

// https://core.telegram.org/bots/api#reactiontypeemoji
#[derive(Debug, Serialize, Deserialize)]
struct ReactionTypeEmoji {
    // type: String,
    emoji: String,
}

// https://core.telegram.org/bots/api#reactiontypecustomemoji
#[derive(Debug, Serialize, Deserialize)]
struct ReactionTypeCustomEmoji {
    // type: String,
    custom_emoji_id: String,
}

// https://core.telegram.org/bots/api#reactioncount
#[derive(Debug, Serialize, Deserialize)]
struct ReactionCount {
    // type: ReactionType,
    total_count: i64,
}

// https://core.telegram.org/bots/api#messagereactionupdated
#[derive(Debug, Serialize, Deserialize)]
struct MessageReactionUpdated {
    chat: Chat,
    message_id: i64,
    user: Option<User>,
    actor_chat: Option<Chat>,
    date: i64,
    old_reaction: Vec<ReactionType>,
    new_reaction: Vec<ReactionType>,
}

// https://core.telegram.org/bots/api#messagereactioncountupdated
#[derive(Debug, Serialize, Deserialize)]
struct MessageReactionCountUpdated {
    chat: Chat,
    message_id: i64,
    date: i64,
    reactions: Vec<ReactionCount>,
}

// https://core.telegram.org/bots/api#forumtopic
#[derive(Debug, Serialize, Deserialize)]
struct ForumTopic {
    message_thread_id: i64,
    name: String,
    icon_color: i64,
    icon_custom_emoji_id: Option<String>,
}

// https://core.telegram.org/bots/api#botcommand
#[derive(Debug, Serialize, Deserialize)]
struct BotCommand {
    command: String,
    description: String,
}

// https://core.telegram.org/bots/api#botcommandscope
#[derive(Debug, Serialize, Deserialize)]
struct BotCommandScope {}

// https://core.telegram.org/bots/api#botcommandscopedefault
#[derive(Debug, Serialize, Deserialize)]
struct BotCommandScopeDefault {
    // type: String,
}

// https://core.telegram.org/bots/api#botcommandscopeallprivatechats
#[derive(Debug, Serialize, Deserialize)]
struct BotCommandScopeAllPrivateChats {
    // type: String,
}

// https://core.telegram.org/bots/api#botcommandscopeallgroupchats
#[derive(Debug, Serialize, Deserialize)]
struct BotCommandScopeAllGroupChats {
    // type: String,
}

// https://core.telegram.org/bots/api#botcommandscopeallchatadministrators
#[derive(Debug, Serialize, Deserialize)]
struct BotCommandScopeAllChatAdministrators {
    // type: String,
}

// https://core.telegram.org/bots/api#botcommandscopechat
#[derive(Debug, Serialize, Deserialize)]
struct BotCommandScopeChat {
    // type: String,
    chat_id: i64,
}

// https://core.telegram.org/bots/api#botcommandscopechatadministrators
#[derive(Debug, Serialize, Deserialize)]
struct BotCommandScopeChatAdministrators {
    // type: String,
    chat_id: i64,
}

// https://core.telegram.org/bots/api#botcommandscopechatmember
#[derive(Debug, Serialize, Deserialize)]
struct BotCommandScopeChatMember {
    // type: String,
    // chat_id: i64,
    user_id: i64,
}

// https://core.telegram.org/bots/api#botname
#[derive(Debug, Serialize, Deserialize)]
struct BotName {
    name: String,
}

// https://core.telegram.org/bots/api#botdescription
#[derive(Debug, Serialize, Deserialize)]
struct BotDescription {
    description: String,
}

// https://core.telegram.org/bots/api#botshortdescription
#[derive(Debug, Serialize, Deserialize)]
struct BotShortDescription {
    short_description: String,
}

// https://core.telegram.org/bots/api#menubutton
#[derive(Debug, Serialize, Deserialize)]
struct MenuButton {}

// https://core.telegram.org/bots/api#menubuttoncommands
#[derive(Debug, Serialize, Deserialize)]
struct MenuButtonCommands {
    // type: String,
}

// https://core.telegram.org/bots/api#menubuttonwebapp
#[derive(Debug, Serialize, Deserialize)]
struct MenuButtonWebApp {
    // type: String,
    text: String,
    web_app: WebAppInfo,
}

// https://core.telegram.org/bots/api#menubuttondefault
#[derive(Debug, Serialize, Deserialize)]
struct MenuButtonDefault {
    // type: String,
}

// https://core.telegram.org/bots/api#chatboostsource
#[derive(Debug, Serialize, Deserialize)]
struct ChatBoostSource {}

// https://core.telegram.org/bots/api#chatboostsourcepremium
#[derive(Debug, Serialize, Deserialize)]
struct ChatBoostSourcePremium {
    source: String,
    user: User,
}

// https://core.telegram.org/bots/api#chatboostsourcegiftcode
#[derive(Debug, Serialize, Deserialize)]
struct ChatBoostSourceGiftCode {
    source: String,
    user: User,
}

// https://core.telegram.org/bots/api#chatboostsourcegiveaway
#[derive(Debug, Serialize, Deserialize)]
struct ChatBoostSourceGiveaway {
    source: String,
    giveaway_message_id: i64,
    user: Option<User>,
    is_unclaimed: Option<bool>,
}

// https://core.telegram.org/bots/api#chatboost
#[derive(Debug, Serialize, Deserialize)]
struct ChatBoost {
    boost_id: String,
    add_date: i64,
    expiration_date: i64,
    source: ChatBoostSource,
}

// https://core.telegram.org/bots/api#chatboostupdated
#[derive(Debug, Serialize, Deserialize)]
struct ChatBoostUpdated {
    chat: Chat,
    boost: ChatBoost,
}

// https://core.telegram.org/bots/api#chatboostremoved
#[derive(Debug, Serialize, Deserialize)]
struct ChatBoostRemoved {
    chat: Chat,
    boost_id: String,
    remove_date: i64,
    source: ChatBoostSource,
}

// https://core.telegram.org/bots/api#userchatboosts
#[derive(Debug, Serialize, Deserialize)]
struct UserChatBoosts {
    boosts: Vec<ChatBoost>,
}

// https://core.telegram.org/bots/api#responseparameters
#[derive(Debug, Serialize, Deserialize)]
struct ResponseParameters {
    migrate_to_chat_id: Option<i64>,
    retry_after: Option<i64>,
}

// https://core.telegram.org/bots/api#inputmedia
#[derive(Debug, Serialize, Deserialize)]
struct InputMedia {}

// https://core.telegram.org/bots/api#inputmediaphoto
#[derive(Debug, Serialize, Deserialize)]
struct InputMediaPhoto {
    // type: String,
    media: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    has_spoiler: Option<bool>,
}

// https://core.telegram.org/bots/api#inputmediavideo
#[derive(Debug, Serialize, Deserialize)]
struct InputMediaVideo {
    // type: String,
    media: String,
    // thumbnail: Option<InputFile or String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    width: Option<i64>,
    height: Option<i64>,
    duration: Option<i64>,
    supports_streaming: Option<bool>,
    has_spoiler: Option<bool>,
}

// https://core.telegram.org/bots/api#inputmediaanimation
#[derive(Debug, Serialize, Deserialize)]
struct InputMediaAnimation {
    // type: String,
    media: String,
    // thumbnail: Option<InputFile or String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    width: Option<i64>,
    height: Option<i64>,
    duration: Option<i64>,
    has_spoiler: Option<bool>,
}

// https://core.telegram.org/bots/api#inputmediaaudio
#[derive(Debug, Serialize, Deserialize)]
struct InputMediaAudio {
    // type: String,
    media: String,
    // thumbnail: Option<InputFile or String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    duration: Option<i64>,
    performer: Option<String>,
    title: Option<String>,
}

// https://core.telegram.org/bots/api#inputmediadocument
#[derive(Debug, Serialize, Deserialize)]
struct InputMediaDocument {
    // type: String,
    media: String,
    // thumbnail: Option<InputFile or String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_content_type_detection: Option<bool>,
}

// https://core.telegram.org/bots/api#inputfile
#[derive(Debug, Serialize, Deserialize)]
pub struct InputFile {}

// https://core.telegram.org/bots/api#sticker
#[derive(Debug, Serialize, Deserialize)]
struct Sticker {
    file_id: String,
    file_unique_id: String,
    // type: String,
    width: i64,
    height: i64,
    is_animated: bool,
    is_video: bool,
    thumbnail: Option<PhotoSize>,
    emoji: Option<String>,
    set_name: Option<String>,
    premium_animation: Option<File>,
    mask_position: Option<MaskPosition>,
    custom_emoji_id: Option<String>,
    needs_repainting: Option<bool>,
    file_size: Option<i64>,
}

// https://core.telegram.org/bots/api#stickerset
#[derive(Debug, Serialize, Deserialize)]
struct StickerSet {
    name: String,
    title: String,
    sticker_type: String,
    is_animated: bool,
    is_video: bool,
    stickers: Vec<Sticker>,
    thumbnail: Option<PhotoSize>,
}

// https://core.telegram.org/bots/api#maskposition
#[derive(Debug, Serialize, Deserialize)]
struct MaskPosition {
    point: String,
    x_shift: f64,
    y_shift: f64,
    scale: f64,
}

// https://core.telegram.org/bots/api#inputsticker
#[derive(Debug, Serialize, Deserialize)]
struct InputSticker {
    // sticker: InputFile or String,
    emoji_list: Vec<String>,
    mask_position: Option<MaskPosition>,
    keywords: Option<Vec<String>>,
}

// https://core.telegram.org/bots/api#inlinequery
#[derive(Debug, Serialize, Deserialize)]
struct InlineQuery {
    id: String,
    from: User,
    query: String,
    offset: String,
    chat_type: Option<String>,
    location: Option<Location>,
}

// https://core.telegram.org/bots/api#inlinequeryresultsbutton
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultsButton {
    text: String,
    web_app: Option<WebAppInfo>,
    start_parameter: Option<String>,
}

// https://core.telegram.org/bots/api#inlinequeryresult
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResult {}

// https://core.telegram.org/bots/api#inlinequeryresultarticle
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultArticle {
    // type: String,
    id: String,
    title: String,
    input_message_content: InputMessageContent,
    reply_markup: Option<InlineKeyboardMarkup>,
    url: Option<String>,
    hide_url: Option<bool>,
    description: Option<String>,
    thumbnail_url: Option<String>,
    thumbnail_width: Option<i64>,
    thumbnail_height: Option<i64>,
}

// https://core.telegram.org/bots/api#inlinequeryresultphoto
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultPhoto {
    // type: String,
    id: String,
    photo_url: String,
    thumbnail_url: String,
    photo_width: Option<i64>,
    photo_height: Option<i64>,
    title: Option<String>,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inlinequeryresultgif
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultGif {
    // type: String,
    id: String,
    gif_url: String,
    gif_width: Option<i64>,
    gif_height: Option<i64>,
    gif_duration: Option<i64>,
    thumbnail_url: String,
    thumbnail_mime_type: Option<String>,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inlinequeryresultmpeg4gif
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultMpeg4Gif {
    // type: String,
    id: String,
    mpeg4_url: String,
    mpeg4_width: Option<i64>,
    mpeg4_height: Option<i64>,
    mpeg4_duration: Option<i64>,
    thumbnail_url: String,
    thumbnail_mime_type: Option<String>,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inlinequeryresultvideo
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultVideo {}

// https://core.telegram.org/bots/api#inlinequeryresultaudio
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultAudio {
    // type: String,
    id: String,
    audio_url: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    performer: Option<String>,
    audio_duration: Option<i64>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inlinequeryresultvoice
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultVoice {
    // type: String,
    id: String,
    voice_url: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    voice_duration: Option<i64>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inlinequeryresultdocument
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultDocument {
    // type: String,
    id: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    document_url: String,
    mime_type: String,
    description: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumbnail_url: Option<String>,
    thumbnail_width: Option<i64>,
    thumbnail_height: Option<i64>,
}

// https://core.telegram.org/bots/api#inlinequeryresultlocation
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultLocation {
    // type: String,
    id: String,
    latitude: f64,
    longitude: f64,
    title: String,
    horizontal_accuracy: Option<f64>,
    live_period: Option<i64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumbnail_url: Option<String>,
    thumbnail_width: Option<i64>,
    thumbnail_height: Option<i64>,
}

// https://core.telegram.org/bots/api#inlinequeryresultvenue
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultVenue {
    // type: String,
    id: String,
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumbnail_url: Option<String>,
    thumbnail_width: Option<i64>,
    thumbnail_height: Option<i64>,
}

// https://core.telegram.org/bots/api#inlinequeryresultcontact
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultContact {
    // type: String,
    id: String,
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumbnail_url: Option<String>,
    thumbnail_width: Option<i64>,
    thumbnail_height: Option<i64>,
}

// https://core.telegram.org/bots/api#inlinequeryresultgame
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultGame {
    // type: String,
    id: String,
    game_short_name: String,
    reply_markup: Option<InlineKeyboardMarkup>,
}

// https://core.telegram.org/bots/api#inlinequeryresultcachedphoto
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultCachedPhoto {
    // type: String,
    id: String,
    photo_file_id: String,
    title: Option<String>,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inlinequeryresultcachedgif
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultCachedGif {
    // type: String,
    id: String,
    gif_file_id: String,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inlinequeryresultcachedmpeg4gif
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultCachedMpeg4Gif {
    // type: String,
    id: String,
    mpeg4_file_id: String,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inlinequeryresultcachedsticker
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultCachedSticker {
    // type: String,
    id: String,
    sticker_file_id: String,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inlinequeryresultcacheddocument
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultCachedDocument {
    // type: String,
    id: String,
    title: String,
    document_file_id: String,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inlinequeryresultcachedvideo
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultCachedVideo {
    // type: String,
    id: String,
    video_file_id: String,
    title: String,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inlinequeryresultcachedvoice
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultCachedVoice {
    // type: String,
    id: String,
    voice_file_id: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inlinequeryresultcachedaudio
#[derive(Debug, Serialize, Deserialize)]
struct InlineQueryResultCachedAudio {
    // type: String,
    id: String,
    audio_file_id: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

// https://core.telegram.org/bots/api#inputmessagecontent
#[derive(Debug, Serialize, Deserialize)]
struct InputMessageContent {}

// https://core.telegram.org/bots/api#inputtextmessagecontent
#[derive(Debug, Serialize, Deserialize)]
struct InputTextMessageContent {
    message_text: String,
    parse_mode: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    link_preview_options: Option<LinkPreviewOptions>,
}

// https://core.telegram.org/bots/api#inputlocationmessagecontent
#[derive(Debug, Serialize, Deserialize)]
struct InputLocationMessageContent {
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<i64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
}

// https://core.telegram.org/bots/api#inputvenuemessagecontent
#[derive(Debug, Serialize, Deserialize)]
struct InputVenueMessageContent {
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
}

// https://core.telegram.org/bots/api#inputcontactmessagecontent
#[derive(Debug, Serialize, Deserialize)]
struct InputContactMessageContent {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
}

// https://core.telegram.org/bots/api#inputinvoicemessagecontent
#[derive(Debug, Serialize, Deserialize)]
struct InputInvoiceMessageContent {
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    max_tip_amount: Option<i64>,
    suggested_tip_amounts: Option<Vec<i64>>,
    provider_data: Option<String>,
    photo_url: Option<String>,
    photo_size: Option<i64>,
    photo_width: Option<i64>,
    photo_height: Option<i64>,
    need_name: Option<bool>,
    need_phone_number: Option<bool>,
    need_email: Option<bool>,
    need_shipping_address: Option<bool>,
    send_phone_number_to_provider: Option<bool>,
    send_email_to_provider: Option<bool>,
    is_flexible: Option<bool>,
}

// https://core.telegram.org/bots/api#choseninlineresult
#[derive(Debug, Serialize, Deserialize)]
struct ChosenInlineResult {
    result_id: String,
    from: User,
    location: Option<Location>,
    inline_message_id: Option<String>,
    query: String,
}

// https://core.telegram.org/bots/api#sentwebappmessage
#[derive(Debug, Serialize, Deserialize)]
struct SentWebAppMessage {
    inline_message_id: Option<String>,
}

// https://core.telegram.org/bots/api#labeledprice
#[derive(Debug, Serialize, Deserialize)]
struct LabeledPrice {
    label: String,
    amount: i64,
}

// https://core.telegram.org/bots/api#invoice
#[derive(Debug, Serialize, Deserialize)]
struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: i64,
}

// https://core.telegram.org/bots/api#shippingaddress
#[derive(Debug, Serialize, Deserialize)]
struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}

// https://core.telegram.org/bots/api#orderinfo
#[derive(Debug, Serialize, Deserialize)]
struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<ShippingAddress>,
}

// https://core.telegram.org/bots/api#shippingoption
#[derive(Debug, Serialize, Deserialize)]
struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}

// https://core.telegram.org/bots/api#successfulpayment
#[derive(Debug, Serialize, Deserialize)]
struct SuccessfulPayment {
    currency: String,
    total_amount: i64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
    telegram_payment_charge_id: String,
    provider_payment_charge_id: String,
}

// https://core.telegram.org/bots/api#shippingquery
#[derive(Debug, Serialize, Deserialize)]
struct ShippingQuery {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}

// https://core.telegram.org/bots/api#precheckoutquery
#[derive(Debug, Serialize, Deserialize)]
struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: i64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}

// https://core.telegram.org/bots/api#passportdata
#[derive(Debug, Serialize, Deserialize)]
struct PassportData {
    data: Vec<EncryptedPassportElement>,
    credentials: EncryptedCredentials,
}

// https://core.telegram.org/bots/api#passportfile
#[derive(Debug, Serialize, Deserialize)]
struct PassportFile {
    file_id: String,
    file_unique_id: String,
    file_size: i64,
    file_date: i64,
}

// https://core.telegram.org/bots/api#encryptedpassportelement
#[derive(Debug, Serialize, Deserialize)]
struct EncryptedPassportElement {
    // type: String,
    data: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    files: Option<Vec<PassportFile>>,
    front_side: Option<PassportFile>,
    reverse_side: Option<PassportFile>,
    selfie: Option<PassportFile>,
    translation: Option<Vec<PassportFile>>,
    hash: String,
}

// https://core.telegram.org/bots/api#encryptedcredentials
#[derive(Debug, Serialize, Deserialize)]
struct EncryptedCredentials {
    data: String,
    hash: String,
    secret: String,
}

// https://core.telegram.org/bots/api#passportelementerror
#[derive(Debug, Serialize, Deserialize)]
struct PassportElementError {}

// https://core.telegram.org/bots/api#passportelementerrordatafield
#[derive(Debug, Serialize, Deserialize)]
struct PassportElementErrorDataField {
    source: String,
    // type: String,
    field_name: String,
    data_hash: String,
    message: String,
}

// https://core.telegram.org/bots/api#passportelementerrorfrontside
#[derive(Debug, Serialize, Deserialize)]
struct PassportElementErrorFrontSide {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}

// https://core.telegram.org/bots/api#passportelementerrorreverseside
#[derive(Debug, Serialize, Deserialize)]
struct PassportElementErrorReverseSide {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}

// https://core.telegram.org/bots/api#passportelementerrorselfie
#[derive(Debug, Serialize, Deserialize)]
struct PassportElementErrorSelfie {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}

// https://core.telegram.org/bots/api#passportelementerrorfile
#[derive(Debug, Serialize, Deserialize)]
struct PassportElementErrorFile {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}

// https://core.telegram.org/bots/api#passportelementerrorfiles
#[derive(Debug, Serialize, Deserialize)]
struct PassportElementErrorFiles {
    source: String,
    // type: String,
    file_hashes: Vec<String>,
    message: String,
}

// https://core.telegram.org/bots/api#passportelementerrortranslationfile
#[derive(Debug, Serialize, Deserialize)]
struct PassportElementErrorTranslationFile {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}

// https://core.telegram.org/bots/api#passportelementerrortranslationfiles
#[derive(Debug, Serialize, Deserialize)]
struct PassportElementErrorTranslationFiles {
    source: String,
    // type: String,
    file_hashes: Vec<String>,
    message: String,
}

// https://core.telegram.org/bots/api#passportelementerrorunspecified
#[derive(Debug, Serialize, Deserialize)]
struct PassportElementErrorUnspecified {
    source: String,
    // type: String,
    element_hash: String,
    message: String,
}

// https://core.telegram.org/bots/api#game
#[derive(Debug, Serialize, Deserialize)]
struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
}

// https://core.telegram.org/bots/api#callbackgame
#[derive(Debug, Serialize, Deserialize)]
struct CallbackGame {}

// https://core.telegram.org/bots/api#gamehighscore
#[derive(Debug, Serialize, Deserialize)]
struct GameHighScore {
    position: i64,
    user: User,
    score: i64,
}
