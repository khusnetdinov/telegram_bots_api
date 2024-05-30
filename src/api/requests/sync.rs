use crate::api::enums::chat_member::ChatMember;
use crate::api::enums::menu_button::MenuButton;
use crate::api::enums::message_result::MessageResult;
use crate::api::params::add_sticker_to_set::AddStickerToSet;
use crate::api::params::answer_callback_query::AnswerCallbackQuery;
use crate::api::params::answer_inline_query::AnswerInlineQuery;
use crate::api::params::answer_pre_checkout_query::AnswerPreCheckoutQuery;
use crate::api::params::answer_shipping_query::AnswerShippingQuery;
use crate::api::params::answer_web_app_query::AnswerWebAppQuery;
use crate::api::params::approve_chat_join_request::ApproveChatJoinRequest;
use crate::api::params::ban_chat_member::BanChatMember;
use crate::api::params::ban_chat_sender_chat::BanChatSenderChat;
use crate::api::params::close_forum_topic::CloseForumTopic;
use crate::api::params::close_general_forum_topic::CloseGeneralForumTopic;
use crate::api::params::copy_message::CopyMessage;
use crate::api::params::copy_messages::CopyMessages;
use crate::api::params::create_chat_invite_link::CreateChatInviteLink;
use crate::api::params::create_forum_topic::CreateForumTopic;
use crate::api::params::create_invoice_link::CreateInvoiceLink;
use crate::api::params::create_new_sticker_set::CreateNewStickerSet;
use crate::api::params::decline_chat_join_request::DeclineChatJoinRequest;
use crate::api::params::delete_chat_photo::DeleteChatPhoto;
use crate::api::params::delete_chat_sticker_set::DeleteChatStickerSet;
use crate::api::params::delete_forum_topic::DeleteForumTopic;
use crate::api::params::delete_message::DeleteMessage;
use crate::api::params::delete_messages::DeleteMessages;
use crate::api::params::delete_my_commands::DeleteMyCommands;
use crate::api::params::delete_sticker_from_set::DeleteStickerFromSet;
use crate::api::params::delete_sticker_set::DeleteStickerSet;
use crate::api::params::delete_webhook::DeleteWebhook;
use crate::api::params::edit_chat_invite_link::EditChatInviteLink;
use crate::api::params::edit_forum_topic::EditForumTopic;
use crate::api::params::edit_general_forum_topic::EditGeneralForumTopic;
use crate::api::params::edit_message_caption::EditMessageCaption;
use crate::api::params::edit_message_live_location::EditMessageLiveLocation;
use crate::api::params::edit_message_media::EditMessageMedia;
use crate::api::params::edit_message_reply_markup::EditMessageReplyMarkup;
use crate::api::params::edit_message_text::EditMessageText;
use crate::api::params::export_chat_invite_link::ExportChatInviteLink;
use crate::api::params::forward_message::ForwardMessage;
use crate::api::params::forward_messages::ForwardMessages;
use crate::api::params::get_business_connection::GetBusinessConnection;
use crate::api::params::get_chat::GetChat;
use crate::api::params::get_chat_administrators::GetChatAdministrators;
use crate::api::params::get_chat_member::GetChatMember;
use crate::api::params::get_chat_member_count::GetChatMemberCount;
use crate::api::params::get_chat_menu_button::GetChatMenuButton;
use crate::api::params::get_custom_emoji_stickers::GetCustomEmojiStickers;
use crate::api::params::get_file::GetFile;
use crate::api::params::get_forum_topic_icon_stickers::GetForumTopicIconStickers;
use crate::api::params::get_game_high_scores::GetGameHighScores;
use crate::api::params::get_my_commands::GetMyCommands;
use crate::api::params::get_my_default_administrator_rights::GetMyDefaultAdministratorRights;
use crate::api::params::get_my_description::GetMyDescription;
use crate::api::params::get_my_name::GetMyName;
use crate::api::params::get_my_short_description::GetMyShortDescription;
use crate::api::params::get_sticker_set::GetStickerSet;
use crate::api::params::get_update::GetUpdate;
use crate::api::params::get_user_chat_boosts::GetUserChatBoosts;
use crate::api::params::get_user_profile_photos::GetUserProfilePhotos;
use crate::api::params::hide_general_forum_topic::HideGeneralForumTopic;
use crate::api::params::leave_chat::LeaveChat;
use crate::api::params::pin_chat_message::PinChatMessage;
use crate::api::params::promote_chat_member::PromoteChatMember;
use crate::api::params::reopen_forum_topic::ReopenForumTopic;
use crate::api::params::reopen_general_forum_topic::ReopenGeneralForumTopic;
use crate::api::params::replace_sticker_in_set::ReplaceStickerInSet;
use crate::api::params::restrict_chat_member::RestrictChatMember;
use crate::api::params::revoke_chat_invite_link::RevokeChatInviteLink;
use crate::api::params::send_animation::SendAnimation;
use crate::api::params::send_audio::SendAudio;
use crate::api::params::send_chat_action::SendChatAction;
use crate::api::params::send_contact::SendContact;
use crate::api::params::send_dice::SendDice;
use crate::api::params::send_document::SendDocument;
use crate::api::params::send_game::SendGame;
use crate::api::params::send_invoice::SendInvoice;
use crate::api::params::send_location::SendLocation;
use crate::api::params::send_media_group::SendMediaGroup;
use crate::api::params::send_message::SendMessage;
use crate::api::params::send_photo::SendPhoto;
use crate::api::params::send_poll::SendPoll;
use crate::api::params::send_sticker::SendSticker;
use crate::api::params::send_venue::SendVenue;
use crate::api::params::send_video::SendVideo;
use crate::api::params::send_video_note::SendVideoNote;
use crate::api::params::send_voice::SendVoice;
use crate::api::params::set_chat_administrator_custom_title::SetChatAdministratorCustomTitle;
use crate::api::params::set_chat_description::SetChatDescription;
use crate::api::params::set_chat_menu_button::SetChatMenuButton;
use crate::api::params::set_chat_permissions::SetChatPermissions;
use crate::api::params::set_chat_photo::SetChatPhoto;
use crate::api::params::set_chat_sticker_set::SetChatStickerSet;
use crate::api::params::set_chat_title::SetChatTitle;
use crate::api::params::set_custom_emoji_sticker_set_thumbnail::SetCustomEmojiStickerSetThumbnail;
use crate::api::params::set_game_score::SetGameScore;
use crate::api::params::set_message_reaction::SetMessageReaction;
use crate::api::params::set_my_commands::SetMyCommands;
use crate::api::params::set_my_default_administrator_rights::SetMyDefaultAdministratorRights;
use crate::api::params::set_my_description::SetMyDescription;
use crate::api::params::set_my_name::SetMyName;
use crate::api::params::set_my_short_description::SetMyShortDescription;
use crate::api::params::set_passport_data_errors::SetPassportDataErrors;
use crate::api::params::set_sticker_emoji_list::SetStickerEmojiList;
use crate::api::params::set_sticker_keywords::SetStickerKeywords;
use crate::api::params::set_sticker_mask_position::SetStickerMaskPosition;
use crate::api::params::set_sticker_position_in_set::SetStickerPositionInSet;
use crate::api::params::set_sticker_set_thumbnail::SetStickerSetThumbnail;
use crate::api::params::set_sticker_set_title::SetStickerSetTitle;
use crate::api::params::set_webhook::SetWebhook;
use crate::api::params::stop_message_live_location::StopMessageLiveLocation;
use crate::api::params::stop_poll::StopPoll;
use crate::api::params::unban_chat_member::UnbanChatMember;
use crate::api::params::unban_chat_sender_chat::UnbanChatSenderChat;
use crate::api::params::unhide_general_forum_topic::UnhideGeneralForumTopic;
use crate::api::params::unpin_all_chat_messages::UnpinAllChatMessages;
use crate::api::params::unpin_all_forum_topic_messages::UnpinAllForumTopicMessages;
use crate::api::params::unpin_all_general_forum_topic_messages::UnpinAllGeneralForumTopicMessages;
use crate::api::params::unpin_chat_message::UnpinChatMessage;
use crate::api::params::upload_sticker_file::UploadStickerFile;
use crate::api::structs::bot_command::BotCommand;
use crate::api::structs::bot_description::BotDescription;
use crate::api::structs::bot_name::BotName;
use crate::api::structs::bot_short_description::BotShortDescription;
use crate::api::structs::business_connection::BusinessConnection;
use crate::api::structs::chat_administrator_rights::ChatAdministratorRights;
use crate::api::structs::chat_full_info::ChatFullInfo;
use crate::api::structs::chat_invite_link::ChatInviteLink;
use crate::api::structs::file::File;
use crate::api::structs::forum_topic::ForumTopic;
use crate::api::structs::game_high_score::GameHighScore;
use crate::api::structs::message::Message;
use crate::api::structs::message_id::MessageId;
use crate::api::structs::poll::Poll;
use crate::api::structs::sent_web_app_message::SentWebAppMessage;
use crate::api::structs::sticker::Sticker;
use crate::api::structs::sticker_set::StickerSet;
use crate::api::structs::update::Update;
use crate::api::structs::user::User;
use crate::api::structs::user_chat_boosts::UserChatBoosts;
use crate::api::structs::user_profile_photos::UserProfilePhotos;
use crate::api::structs::webhook_info::WebhookInfo;
use crate::errors::Error;

/// Sync client methods for telegram bots api.
pub trait Requests {
    /// <https://core.telegram.org/bots/api#getupdates>
    /// Use this method to receive incoming updates using long polling (wiki). Returns an Array of Update objects.
    fn get_updates(&self, params: &GetUpdate) -> Result<Vec<Update>, Error>;

    /// <https://core.telegram.org/bots/api#setwebhook>
    /// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.
    /// If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter secret_token. If specified, the request will contain a header “X-Telegram-Bot-Api-Secret-Token” with the secret token as content.
    fn set_webhook(&self, params: &SetWebhook) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#deletewebhook>
    /// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success.
    fn delete_webhook(&self, params: &DeleteWebhook) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#getwebhookinfo>
    /// Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty.
    fn get_webhook_info(&self) -> Result<WebhookInfo, Error>;

    /// <https://core.telegram.org/bots/api#getme>
    /// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object.
    fn get_me(&self) -> Result<User, Error>;

    /// <https://core.telegram.org/bots/api#logout>
    /// Use this method to log out from the cloud Bot API server before launching the bot locally. You must log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns True on success. Requires no parameters.
    fn log_out(&self) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#close>
    /// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns True on success. Requires no parameters.
    fn close(&self) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#sendmessage>
    /// Use this method to send text messages. On success, the sent Message is returned.
    fn send_message(&self, params: &SendMessage) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#forwardmessage>
    /// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent Message is returned.
    fn forward_message(&self, params: &ForwardMessage) -> Result<MessageId, Error>;

    /// <https://core.telegram.org/bots/api#forwardmessages>
    /// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of MessageId of the sent messages is returned.
    fn forward_messages(&self, params: &ForwardMessages) -> Result<Vec<MessageId>, Error>;

    /// <https://core.telegram.org/bots/api#copymessage>
    /// Use this method to copy messages of any kind. Service messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessage, but the copied message doesn't have a link to the original message. Returns the MessageId of the sent message on success.
    fn copy_message(&self, params: &CopyMessage) -> Result<MessageId, Error>;

    /// <https://core.telegram.org/bots/api#copymessages>
    /// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessages, but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of MessageId of the sent messages is returned.
    fn copy_messages(&self, params: &CopyMessages) -> Result<Vec<MessageId>, Error>;

    /// <https://core.telegram.org/bots/api#sendphoto>
    /// Use this method to send photos. On success, the sent Message is returned.
    fn send_photo(&self, params: &SendPhoto) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#sendaudio>
    /// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
    fn send_audio(&self, params: &SendAudio) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#senddocument>
    /// Use this method to send general files. On success, the sent Message is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
    fn send_document(&self, params: &SendDocument) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#sendvideo>
    /// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
    fn send_video(&self, params: &SendVideo) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#sendanimation>
    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    fn send_animation(&self, params: &SendAnimation) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#sendvoice>
    /// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS (other formats may be sent as Audio or Document). On success, the sent Message is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
    fn send_voice(&self, params: &SendVoice) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#sendvideonote>
    /// As of v.4.0, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned.
    fn send_video_note(&self, params: &SendVideoNote) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#sendmediagroup>
    /// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Messages that were sent is returned.
    fn send_media_group(&self, params: &SendMediaGroup) -> Result<Vec<Message>, Error>;

    /// <https://core.telegram.org/bots/api#sendlocation>
    /// Use this method to send point on the map. On success, the sent Message is returned.
    fn send_location(&self, params: &SendLocation) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#sendvenue>
    /// Use this method to send information about a venue. On success, the sent Message is returned.
    fn send_venue(&self, params: &SendVenue) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#sendcontact>
    /// Use this method to send phone contacts. On success, the sent Message is returned.
    fn send_contact(&self, params: &SendContact) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#sendpoll>
    /// Use this method to send a native poll. On success, the sent Message is returned.
    fn send_poll(&self, params: &SendPoll) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#senddice>
    /// Use this method to send an animated emoji that will display a random value. On success, the sent Message is returned.
    fn send_dice(&self, params: &SendDice) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#sendchataction>
    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.
    fn send_chat_action(&self, params: &SendChatAction) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setmessagereaction>
    /// Use this method to change the chosen reactions on a message. Service messages can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Returns True on success.
    fn set_message_reaction(&self, params: &SetMessageReaction) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#getuserprofilephotos>
    /// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
    fn get_user_profile_photos(
        &self,
        params: &GetUserProfilePhotos,
    ) -> Result<UserProfilePhotos, Error>;

    /// <https://core.telegram.org/bots/api#getfile>
    /// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link <https://api.telegram.org/file/bot#{token}/{file_path}>, where file_path is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.
    fn get_file(&self, params: &GetFile) -> Result<File, Error>;

    /// <https://core.telegram.org/bots/api#banchatmember>
    /// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    fn ban_chat_member(&self, params: &BanChatMember) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#unbanchatmember>
    /// Use this method to unban a previously banned user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be removed from the chat. If you don't want this, use the parameter only_if_banned. Returns True on success.
    fn unban_chat_member(&self, params: &UnbanChatMember) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#restrictchatmember>
    /// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass True for all permissions to lift restrictions from a user. Returns True on success.
    fn restrict_chat_member(&self, params: &RestrictChatMember) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#promotechatmember>
    /// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass False for all boolean parameters to demote a user. Returns True on success.
    fn promote_chat_member(&self, params: &PromoteChatMember) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setchatadministratorcustomtitle>
    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns True on success.
    fn set_chat_administrator_custom_title(
        &self,
        params: &SetChatAdministratorCustomTitle,
    ) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#banchatsenderchat>
    /// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is unbanned, the owner of the banned chat won't be able to send messages on behalf of any of their channels. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns True on success.
    fn ban_chat_sender_chat(&self, params: &BanChatSenderChat) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#unbanchatsenderchat>
    /// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns True on success.
    fn unban_chat_sender_chat(&self, params: &UnbanChatSenderChat) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setchatpermissions>
    /// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members administrator rights. Returns True on success.
    fn set_chat_permissions(&self, params: &SetChatPermissions) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#exportchatinvitelink>
    /// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as String on success.
    fn export_chat_invite_link(&self, params: &ExportChatInviteLink) -> Result<String, Error>;

    /// <https://core.telegram.org/bots/api#createchatinvitelink>
    /// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method revokeChatInviteLink. Returns the new invite link as ChatInviteLink object.
    fn create_chat_invite_link(
        &self,
        params: &CreateChatInviteLink,
    ) -> Result<ChatInviteLink, Error>;

    /// <https://core.telegram.org/bots/api#editchatinvitelink>
    /// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a ChatInviteLink object.
    fn edit_chat_invite_link(&self, params: &EditChatInviteLink) -> Result<ChatInviteLink, Error>;

    /// <https://core.telegram.org/bots/api#revokechatinvitelink>
    /// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as ChatInviteLink object.
    fn revoke_chat_invite_link(
        &self,
        params: &RevokeChatInviteLink,
    ) -> Result<ChatInviteLink, Error>;

    /// <https://core.telegram.org/bots/api#approvechatjoinrequest>
    /// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
    fn approve_chat_join_request(&self, params: &ApproveChatJoinRequest) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#declinechatjoinrequest>
    /// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
    fn decline_chat_join_request(&self, params: &DeclineChatJoinRequest) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setchatphoto>
    /// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    fn set_chat_photo(&self, params: &SetChatPhoto) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#deletechatphoto>
    /// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    fn delete_chat_photo(&self, params: &DeleteChatPhoto) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setchattitle>
    /// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    fn set_chat_title(&self, params: &SetChatTitle) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setchatdescription>
    /// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    fn set_chat_description(&self, params: &SetChatDescription) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#pinchatmessage>
    /// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
    fn pin_chat_message(&self, params: &PinChatMessage) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#unpinchatmessage>
    /// Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
    fn unpin_chat_message(&self, params: &UnpinChatMessage) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#unpinallchatmessages>
    /// Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
    fn unpin_all_chat_messages(&self, params: &UnpinAllChatMessages) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#leavechat>
    /// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
    fn leave_chat(&self, params: &LeaveChat) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#getchat>
    /// Use this method to get up to date information about the chat. Returns a Chat object on success.
    fn get_chat(&self, params: &GetChat) -> Result<ChatFullInfo, Error>;

    /// <https://core.telegram.org/bots/api#getchatadministrators>
    /// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of ChatMember objects.
    fn get_chat_administrators(
        &self,
        params: &GetChatAdministrators,
    ) -> Result<Vec<ChatMember>, Error>;

    /// <https://core.telegram.org/bots/api#getchatmembercount>
    /// Use this method to get the number of members in a chat. Returns Int on success.
    fn get_chat_member_count(&self, params: &GetChatMemberCount) -> Result<i64, Error>;

    /// <https://core.telegram.org/bots/api#getchatmember>
    /// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a ChatMember object on success.
    fn get_chat_member(&self, params: &GetChatMember) -> Result<ChatMember, Error>;

    /// <https://core.telegram.org/bots/api#setchatstickerset>
    /// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
    fn set_chat_sticker_set(&self, params: &SetChatStickerSet) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#deletechatstickerset>
    /// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
    fn delete_chat_sticker_set(&self, params: &DeleteChatStickerSet) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#getforumtopiciconstickers>
    /// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of Sticker objects.
    fn get_forum_topic_icon_stickers(
        &self,
        params: &GetForumTopicIconStickers,
    ) -> Result<Vec<Sticker>, Error>;

    /// <https://core.telegram.org/bots/api#createforumtopic>
    /// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns information about the created topic as a ForumTopic object.
    fn create_forum_topic(&self, params: &CreateForumTopic) -> Result<ForumTopic, Error>;

    /// <https://core.telegram.org/bots/api#editforumtopic>
    /// Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
    fn edit_forum_topic(&self, params: &EditForumTopic) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#closeforumtopic>
    /// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
    fn close_forum_topic(&self, params: &CloseForumTopic) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#reopenforumtopic>
    /// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
    fn reopen_forum_topic(&self, params: &ReopenForumTopic) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#deleteforumtopic>
    /// Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_delete_messages administrator rights. Returns True on success.
    fn delete_forum_topic(&self, params: &DeleteForumTopic) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#unpinallforumtopicmessages>
    /// Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup. Returns True on success.
    fn unpin_all_forum_topic_messages(
        &self,
        params: &UnpinAllForumTopicMessages,
    ) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#editgeneralforumtopic>
    /// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have can_manage_topics administrator rights. Returns True on success.
    fn edit_general_forum_topic(&self, params: &EditGeneralForumTopic) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#closegeneralforumtopic>
    /// Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success.
    fn close_general_forum_topic(&self, params: &CloseGeneralForumTopic) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#reopengeneralforumtopic>
    /// Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically unhidden if it was hidden. Returns True on success.
    fn reopen_general_forum_topic(&self, params: &ReopenGeneralForumTopic) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#hidegeneralforumtopic>
    /// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically closed if it was open. Returns True on success
    fn hide_general_forum_topic(&self, params: &HideGeneralForumTopic) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#unhidegeneralforumtopic>
    /// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success.
    fn unhide_general_forum_topic(&self, params: &UnhideGeneralForumTopic) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#unpinallgeneralforumtopicmessages>
    /// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup. Returns True on success.
    fn unpin_all_general_forum_topic_messages(
        &self,
        params: &UnpinAllGeneralForumTopicMessages,
    ) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#answercallbackquery>
    /// Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, True is returned.
    fn answer_callback_query(&self, params: &AnswerCallbackQuery) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#getuserchatboosts>
    /// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a UserChatBoosts object.
    fn get_user_chat_boosts(&self, params: &GetUserChatBoosts) -> Result<UserChatBoosts, Error>;

    /// <https://core.telegram.org/bots/api#getbusinessconnection>
    /// Use this method to get information about the connection of the bot with a business account. Returns a BusinessConnection object on success.
    fn get_business_connection(
        &self,
        params: &GetBusinessConnection,
    ) -> Result<BusinessConnection, Error>;

    /// <https://core.telegram.org/bots/api#setmycommands>
    /// Use this method to change the list of the bot's commands. See this manual for more details about bot commands. Returns True on success.
    fn set_my_commands(&self, params: &SetMyCommands) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#deletemycommands>
    /// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, higher level commands will be shown to affected users. Returns True on success.
    fn delete_my_commands(&self, params: &DeleteMyCommands) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#getmycommands>
    /// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of BotCommand objects. If commands aren't set, an empty list is returned.
    fn get_my_commands(&self, params: &GetMyCommands) -> Result<Vec<BotCommand>, Error>;

    /// <https://core.telegram.org/bots/api#setmyname>
    /// Use this method to change the bot's name. Returns True on success.
    fn set_my_name(&self, params: &SetMyName) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#getmyname>
    /// Use this method to get the current bot name for the given user language. Returns BotName on success.
    fn get_my_name(&self, params: &GetMyName) -> Result<BotName, Error>;

    /// <https://core.telegram.org/bots/api#setmydescription>
    /// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns True on success.
    fn set_my_description(&self, params: &SetMyDescription) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#getmydescription>
    /// Use this method to get the current bot description for the given user language. Returns BotDescription on success.
    fn get_my_description(&self, params: &GetMyDescription) -> Result<BotDescription, Error>;

    /// <https://core.telegram.org/bots/api#setmyshortdescription>
    /// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns True on success.
    fn set_my_short_description(&self, params: &SetMyShortDescription) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#getmyshortdescription>
    /// Use this method to get the current bot short description for the given user language. Returns BotShortDescription on success.
    fn get_my_short_description(
        &self,
        params: &GetMyShortDescription,
    ) -> Result<BotShortDescription, Error>;

    /// <https://core.telegram.org/bots/api#setchatmenubutton>
    /// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns True on success.
    fn set_chat_menu_button(&self, params: &SetChatMenuButton) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#getchatmenubutton>
    /// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns MenuButton on success.
    fn get_chat_menu_button(&self, params: &GetChatMenuButton) -> Result<MenuButton, Error>;

    /// <https://core.telegram.org/bots/api#setmydefaultadministratorrights>
    /// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns True on success.
    fn set_my_default_administrator_rights(
        &self,
        params: &SetMyDefaultAdministratorRights,
    ) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#getmydefaultadministratorrights>
    /// Use this method to get the current default administrator rights of the bot. Returns ChatAdministratorRights on success.
    fn get_my_default_administrator_rights(
        &self,
        params: &GetMyDefaultAdministratorRights,
    ) -> Result<ChatAdministratorRights, Error>;

    /// <https://core.telegram.org/bots/api#editmessagetext>
    /// Use this method to edit text and game messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
    fn edit_message_text(&self, params: &EditMessageText) -> Result<MessageResult, Error>;

    /// <https://core.telegram.org/bots/api#editmessagecaption>
    /// Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
    fn edit_message_caption(&self, params: &EditMessageCaption) -> Result<MessageResult, Error>;

    /// <https://core.telegram.org/bots/api#editmessagemedia>
    /// Use this method to edit animation, audio, document, photo, or video messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file_id or specify a URL. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
    fn edit_message_media(&self, params: &EditMessageMedia) -> Result<MessageResult, Error>;

    /// <https://core.telegram.org/bots/api#editmessagelivelocation>
    /// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
    fn edit_message_live_location(
        &self,
        params: &EditMessageLiveLocation,
    ) -> Result<MessageResult, Error>;

    /// <https://core.telegram.org/bots/api#stopmessagelivelocation>
    /// Use this method to stop updating a live location message before live_period expires. On success, if the message is not an inline message, the edited Message is returned, otherwise True is returned.
    fn stop_message_live_location(
        &self,
        params: &StopMessageLiveLocation,
    ) -> Result<MessageResult, Error>;

    /// <https://core.telegram.org/bots/api#editmessagereplymarkup>
    /// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
    fn edit_message_reply_markup(
        &self,
        params: &EditMessageReplyMarkup,
    ) -> Result<MessageResult, Error>;

    /// <https://core.telegram.org/bots/api#stoppoll>
    /// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll is returned.
    fn stop_poll(&self, params: &StopPoll) -> Result<Poll, Error>;

    /// <https://core.telegram.org/bots/api#deletemessage>
    /// Use this method to delete a message, including service messages, with the following limitations:
    /// - A message can only be deleted if it was sent less than 48 hours ago.
    /// - Service messages about a supergroup, channel, or forum topic creation can't be deleted.
    /// - A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.
    /// - Bots can delete outgoing messages in private chats, groups, and supergroups.
    /// - Bots can delete incoming messages in private chats.
    /// - Bots granted can_post_messages permissions can delete outgoing messages in channels.
    /// - If the bot is an administrator of a group, it can delete any message there.
    /// - If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
    /// Returns True on success.
    fn delete_message(&self, params: &DeleteMessage) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#deletemessages>
    /// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns True on success.
    fn delete_messages(&self, params: &DeleteMessages) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#sendsticker>
    /// Use this method to send static .WEBP, animated .TGS, or video .WEBM stickers. On success, the sent Message is returned.
    fn send_sticker(&self, params: &SendSticker) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#getstickerset>
    /// Use this method to get a sticker set. On success, a StickerSet object is returned.
    fn get_sticker_set(&self, params: &GetStickerSet) -> Result<StickerSet, Error>;

    /// <https://core.telegram.org/bots/api#getcustomemojistickers>
    /// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of Sticker objects.
    fn get_custom_emoji_stickers(&self, params: &GetCustomEmojiStickers) -> Result<Sticker, Error>;

    /// <https://core.telegram.org/bots/api#uploadstickerfile>
    /// Use this method to upload a file with a sticker for later use in the createNewStickerSet and addStickerToSet methods (the file can be used multiple times). Returns the uploaded File on success.
    fn upload_sticker_file(&self, params: &UploadStickerFile) -> Result<File, Error>;

    /// <https://core.telegram.org/bots/api#createnewstickerset>
    /// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns True on success.
    fn create_new_sticker_set(&self, params: &CreateNewStickerSet) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#addstickertoset>
    /// Use this method to add a new sticker to a set created by the bot. The format of the added sticker must match the format of the other stickers in the set. Emoji sticker sets can have up to 200 stickers. Animated and video sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers. Returns True on success.
    fn add_sticker_to_set(&self, params: &AddStickerToSet) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setstickerpositioninset>
    /// Use this method to move a sticker in a set created by the bot to a specific position. Returns True on success.
    fn set_sticker_position_in_set(&self, params: &SetStickerPositionInSet) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#deletestickerfromset>
    /// Use this method to delete a sticker from a set created by the bot. Returns True on success.
    fn delete_sticker_from_set(&self, params: &DeleteStickerFromSet) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#replacestickerinset>
    /// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling deleteStickerFromSet, then addStickerToSet, then setStickerPositionInSet. Returns True on success.
    fn replace_sticker_in_set(&self, params: &ReplaceStickerInSet) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setstickeremojilist>
    /// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
    fn set_sticker_emoji_list(&self, params: &SetStickerEmojiList) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setstickerkeywords>
    /// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
    fn set_sticker_keywords(&self, params: &SetStickerKeywords) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setstickermaskposition>
    /// Use this method to change the mask position of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns True on success.
    fn set_sticker_mask_position(&self, params: &SetStickerMaskPosition) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setstickersettitle>
    /// Use this method to set the title of a created sticker set. Returns True on success.
    fn set_sticker_set_title(&self, params: &SetStickerSetTitle) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setstickersetthumbnail>
    /// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns True on success.
    fn set_sticker_set_thumbnail(&self, params: &SetStickerSetThumbnail) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setcustomemojistickersetthumbnail>
    /// Use this method to set the thumbnail of a custom emoji sticker set. Returns True on success.
    fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        params: &SetCustomEmojiStickerSetThumbnail,
    ) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#deletestickerset>
    /// Use this method to delete a sticker set that was created by the bot. Returns True on success.
    fn delete_sticker_set(&self, params: &DeleteStickerSet) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#answerinlinequery>
    /// Use this method to send answers to an inline query. On success, True is returned.
    /// No more than 50 results per query are allowed.
    fn answer_inline_query(&self, params: &AnswerInlineQuery) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#answerwebappquery>
    /// se this method to set the result of an interaction with a Web App and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a SentWebAppMessage object is returned.
    fn answer_web_app_query(&self, params: &AnswerWebAppQuery) -> Result<SentWebAppMessage, Error>;

    /// <https://core.telegram.org/bots/api#sendinvoice>
    /// Use this method to send invoices. On success, the sent Message is returned.
    fn send_invoice(&self, params: &SendInvoice) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#createinvoicelink>
    /// Use this method to create a link for an invoice. Returns the created invoice link as String on success.
    fn create_invoice_link(&self, params: &CreateInvoiceLink) -> Result<String, Error>;

    /// <https://core.telegram.org/bots/api#answershippingquery>
    /// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified, the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned.
    fn answer_shipping_query(&self, params: &AnswerShippingQuery) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#answerprecheckoutquery>
    /// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query. Use this method to respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
    fn answer_pre_checkout_query(&self, params: &AnswerPreCheckoutQuery) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#setpassportdataerrors>
    /// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns True on success.
    /// Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
    fn set_passport_data_errors(&self, params: &SetPassportDataErrors) -> Result<bool, Error>;

    /// <https://core.telegram.org/bots/api#sendgame>
    /// Use this method to send a game. On success, the sent Message is returned.
    fn send_game(&self, params: &SendGame) -> Result<Message, Error>;

    /// <https://core.telegram.org/bots/api#setgamescore>
    /// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the Message is returned, otherwise True is returned. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
    fn set_game_score(&self, params: &SetGameScore) -> Result<MessageResult, Error>;

    /// <https://core.telegram.org/bots/api#getgamehighscores>
    /// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of GameHighScore objects.
    fn get_game_high_scores(&self, params: &GetGameHighScores) -> Result<GameHighScore, Error>;
}
