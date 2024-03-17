use crate::api::enums::chat_member::ChatMember;
use crate::api::enums::menu_buttons::MenuButtons;
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
use crate::api::requests::sync::Requests;
use crate::api::responses::error::ResponseError;
use crate::api::responses::result::ResponseResult;
use crate::api::structs::bot_command::BotCommand;
use crate::api::structs::bot_description::BotDescription;
use crate::api::structs::bot_name::BotName;
use crate::api::structs::bot_short_description::BotShortDescription;
use crate::api::structs::chat::Chat;
use crate::api::structs::chat_administrator_rights::ChatAdministratorRights;
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
use crate::config::Config;
use crate::errors::Error;
use reqwest::blocking::{ClientBuilder, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// Sync client for telegram bots api.
#[derive(Debug)]
pub struct Sync {
    client: reqwest::blocking::Client,
    pub config: Config,
    url: String,
}

impl From<Config> for Sync {
    fn from(config: Config) -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .build()
            .unwrap();

        let url = config.build_url();

        Self {
            client,
            config,
            url,
        }
    }
}

impl Sync {
    pub fn new() -> Self {
        let config = Config::new();
        let url = config.build_url();
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .build()
            .unwrap();

        Self {
            client,
            config,
            url,
        }
    }

    fn request(&self, method: &str) -> RequestBuilder {
        self.client.post(format!("{}{}", self.url, method))
    }

    fn respond_with<T: DeserializeOwned>(&self, response: RequestBuilder) -> Result<T, Error> {
        match response.send() {
            Ok(response) => match response.status().as_u16() {
                200 => self.decode::<T>(response),
                400..=429 => Err(Error::Response(ResponseError::from(
                    &response.text().unwrap(),
                ))),
                _ => Err(Error::Unexpected(String::from(""))),
            },
            Err(error) => Err(Error::Request(error)),
        }
    }

    fn decode<T: DeserializeOwned>(&self, response: Response) -> Result<T, Error> {
        match serde_json::from_str::<ResponseResult<T>>(&response.text().unwrap()) {
            Ok(success) => Ok(success.result),
            Err(error) => Err(Error::Decode(error)),
        }
    }
}

impl Requests for Sync {
    fn get_updates(&self, params: &GetUpdate) -> Result<Vec<Update>, Error> {
        self.respond_with::<Vec<Update>>(self.request("getUpdates").json(params))
    }

    fn set_webhook(&self, params: &SetWebhook) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setWebhook").json(params))
    }

    fn delete_webhook(&self, params: &DeleteWebhook) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("deleteWebhook").json(params))
    }

    fn get_webhook_info(&self) -> Result<WebhookInfo, Error> {
        self.respond_with::<WebhookInfo>(self.request("getWebhookInfo").json(&{}))
    }

    fn get_me(&self) -> Result<User, Error> {
        self.respond_with::<User>(self.request("getMe").json(&{}))
    }

    fn log_out(&self) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("logOut").json(&{}))
    }

    fn close(&self) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("close").json(&{}))
    }

    fn send_message(&self, params: &SendMessage) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendMessage").json(params))
    }

    fn forward_message(&self, params: &ForwardMessage) -> Result<MessageId, Error> {
        self.respond_with::<MessageId>(self.request("forwardMessage").json(params))
    }

    fn forward_messages(&self, params: &ForwardMessages) -> Result<Vec<MessageId>, Error> {
        self.respond_with::<Vec<MessageId>>(self.request("forwardMessages").json(params))
    }

    fn copy_message(&self, params: &CopyMessage) -> Result<MessageId, Error> {
        self.respond_with::<MessageId>(self.request("copyMessage").json(params))
    }

    fn copy_messages(&self, params: &CopyMessages) -> Result<Vec<MessageId>, Error> {
        self.respond_with::<Vec<MessageId>>(self.request("copyMessages").json(params))
    }

    fn send_photo(&self, params: &SendPhoto) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendPhoto").json(params))
    }

    fn send_audio(&self, params: &SendAudio) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendAudio").json(params))
    }

    fn send_document(&self, params: &SendDocument) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendDocument").json(params))
    }

    fn send_video(&self, params: &SendVideo) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendVideo").json(params))
    }

    fn send_animation(&self, params: &SendAnimation) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendAnimation").json(params))
    }

    fn send_voice(&self, params: &SendVoice) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendVoice").json(params))
    }

    fn send_video_note(&self, params: &SendVideoNote) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendVideoNote").json(params))
    }

    fn send_media_group(&self, params: &SendMediaGroup) -> Result<Vec<Message>, Error> {
        self.respond_with::<Vec<Message>>(self.request("sendMediaGroup").json(params))
    }

    fn send_location(&self, params: &SendLocation) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendLocation").json(params))
    }

    fn send_venue(&self, params: &SendVenue) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendVenue").json(params))
    }

    fn send_contact(&self, params: &SendContact) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendContact").json(params))
    }

    fn send_poll(&self, params: &SendPoll) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendPoll").json(params))
    }

    fn send_dice(&self, params: &SendDice) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendDice").json(params))
    }

    fn send_chat_action(&self, params: &SendChatAction) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("sendChatAction").json(params))
    }

    fn set_message_reaction(&self, params: &SetMessageReaction) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setMessageReaction").json(params))
    }

    fn get_user_profile_photos(
        &self,
        params: &GetUserProfilePhotos,
    ) -> Result<UserProfilePhotos, Error> {
        self.respond_with::<UserProfilePhotos>(self.request("getUserProfilePhotos").json(params))
    }

    fn get_file(&self, params: &GetFile) -> Result<File, Error> {
        self.respond_with::<File>(self.request("getFile").json(params))
    }

    fn ban_chat_member(&self, params: &BanChatMember) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("banChatMember").json(params))
    }

    fn unban_chat_member(&self, params: &UnbanChatMember) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("unbanChatMember").json(params))
    }

    fn restrict_chat_member(&self, params: &RestrictChatMember) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("restrictChatMember").json(params))
    }

    fn promote_chat_member(&self, params: &PromoteChatMember) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("promoteChatMember").json(params))
    }

    fn set_chat_administrator_custom_title(
        &self,
        params: &SetChatAdministratorCustomTitle,
    ) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setChatAdministratorCustomTitle").json(params))
    }

    fn ban_chat_sender_chat(&self, params: &BanChatSenderChat) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("banChatSenderChat").json(params))
    }

    fn unban_chat_sender_chat(&self, params: &UnbanChatSenderChat) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("unbanChatSenderChat").json(params))
    }

    fn set_chat_permissions(&self, params: &SetChatPermissions) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setChatPermissions").json(params))
    }

    fn export_chat_invite_link(&self, params: &ExportChatInviteLink) -> Result<String, Error> {
        self.respond_with::<String>(self.request("exportChatInviteLink").json(params))
    }

    fn create_chat_invite_link(
        &self,
        params: &CreateChatInviteLink,
    ) -> Result<ChatInviteLink, Error> {
        self.respond_with::<ChatInviteLink>(self.request("createChatInviteLink").json(params))
    }

    fn edit_chat_invite_link(&self, params: &EditChatInviteLink) -> Result<ChatInviteLink, Error> {
        self.respond_with::<ChatInviteLink>(self.request("editChatInviteLink").json(params))
    }

    fn revoke_chat_invite_link(
        &self,
        params: &RevokeChatInviteLink,
    ) -> Result<ChatInviteLink, Error> {
        self.respond_with::<ChatInviteLink>(self.request("revokeChatInviteLink").json(params))
    }

    fn approve_chat_join_request(&self, params: &ApproveChatJoinRequest) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("approveChatJoinRequest").json(params))
    }

    fn decline_chat_join_request(&self, params: &DeclineChatJoinRequest) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("declineChatJoinRequest").json(params))
    }

    fn set_chat_photo(&self, params: &SetChatPhoto) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setChatPhoto").json(params))
    }

    fn delete_chat_photo(&self, params: &DeleteChatPhoto) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("deleteChatPhoto").json(params))
    }

    fn set_chat_title(&self, params: &SetChatTitle) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setChatTitle").json(params))
    }

    fn set_chat_description(&self, params: &SetChatDescription) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setChatDescription").json(params))
    }

    fn pin_chat_message(&self, params: &PinChatMessage) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("pinChatMessage").json(params))
    }

    fn unpin_chat_message(&self, params: &UnpinChatMessage) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("unpinChatMessage").json(params))
    }

    fn unpin_all_chat_messages(&self, params: &UnpinAllChatMessages) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("unpinAllChatMessages").json(params))
    }

    fn leave_chat(&self, params: &LeaveChat) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("leaveChat").json(params))
    }

    fn get_chat(&self, params: &GetChat) -> Result<Chat, Error> {
        self.respond_with::<Chat>(self.request("getChat").json(params))
    }

    fn get_chat_administrators(
        &self,
        params: &GetChatAdministrators,
    ) -> Result<Vec<ChatMember>, Error> {
        self.respond_with::<Vec<ChatMember>>(self.request("getChatAdministrators").json(params))
    }

    fn get_chat_member_count(&self, params: &GetChatMemberCount) -> Result<i64, Error> {
        self.respond_with::<i64>(self.request("getChatMemberCount").json(params))
    }

    fn get_chat_member(&self, params: &GetChatMember) -> Result<ChatMember, Error> {
        self.respond_with::<ChatMember>(self.request("getChatMember").json(params))
    }

    fn set_chat_sticker_set(&self, params: &SetChatStickerSet) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setChatStickerSet").json(params))
    }

    fn delete_chat_sticker_set(&self, params: &DeleteChatStickerSet) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("deleteChatStickerSet").json(params))
    }

    fn get_forum_topic_icon_stickers(
        &self,
        params: &GetForumTopicIconStickers,
    ) -> Result<Vec<Sticker>, Error> {
        self.respond_with::<Vec<Sticker>>(self.request("getForumTopicIconStickers").json(params))
    }

    fn create_forum_topic(&self, params: &CreateForumTopic) -> Result<ForumTopic, Error> {
        self.respond_with::<ForumTopic>(self.request("createForumTopic").json(params))
    }

    fn edit_forum_topic(&self, params: &EditForumTopic) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("editForumTopic").json(params))
    }

    fn close_forum_topic(&self, params: &CloseForumTopic) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("closeForumTopic").json(params))
    }

    fn reopen_forum_topic(&self, params: &ReopenForumTopic) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("reopenForumTopic").json(params))
    }

    fn delete_forum_topic(&self, params: &DeleteForumTopic) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("deleteForumTopic").json(params))
    }

    fn unpin_all_forum_topic_messages(
        &self,
        params: &UnpinAllForumTopicMessages,
    ) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("unpinAllForumTopicMessages").json(params))
    }

    fn edit_general_forum_topic(&self, params: &EditGeneralForumTopic) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("editGeneralForumTopic").json(params))
    }

    fn close_general_forum_topic(&self, params: &CloseGeneralForumTopic) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("closeGeneralForumTopic").json(params))
    }

    fn reopen_general_forum_topic(&self, params: &ReopenGeneralForumTopic) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("reopenGeneralForumTopic").json(params))
    }

    fn hide_general_forum_topic(&self, params: &HideGeneralForumTopic) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("hideGeneralForumTopic").json(params))
    }

    fn unhide_general_forum_topic(&self, params: &UnhideGeneralForumTopic) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("unhideGeneralForumTopic").json(params))
    }

    fn unpin_all_general_forum_topic_messages(
        &self,
        params: &UnpinAllGeneralForumTopicMessages,
    ) -> Result<bool, Error> {
        self.respond_with::<bool>(
            self.request("unpinAllGeneralForumTopicMessages")
                .json(params),
        )
    }

    fn answer_callback_query(&self, params: &AnswerCallbackQuery) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("answerCallbackQuery").json(params))
    }

    fn get_user_chat_boosts(&self, params: &GetUserChatBoosts) -> Result<UserChatBoosts, Error> {
        self.respond_with::<UserChatBoosts>(self.request("getUserChatBoosts").json(params))
    }

    fn set_my_commands(&self, params: &SetMyCommands) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setMyCommands").json(params))
    }

    fn delete_my_commands(&self, params: &DeleteMyCommands) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("deleteMyCommands").json(params))
    }

    fn get_my_commands(&self, params: &GetMyCommands) -> Result<Vec<BotCommand>, Error> {
        self.respond_with::<Vec<BotCommand>>(self.request("getMyCommands").json(params))
    }

    fn set_my_name(&self, params: &SetMyName) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setMyName").json(params))
    }

    fn get_my_name(&self, params: &GetMyName) -> Result<BotName, Error> {
        self.respond_with::<BotName>(self.request("getMyName").json(params))
    }

    fn set_my_description(&self, params: &SetMyDescription) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setMyDescription").json(params))
    }

    fn get_my_description(&self, params: &GetMyDescription) -> Result<BotDescription, Error> {
        self.respond_with::<BotDescription>(self.request("getMyDescription").json(params))
    }

    fn set_my_short_description(&self, params: &SetMyShortDescription) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setMyShortDescription").json(params))
    }

    fn get_my_short_description(
        &self,
        params: &GetMyShortDescription,
    ) -> Result<BotShortDescription, Error> {
        self.respond_with::<BotShortDescription>(self.request("getMyShortDescription").json(params))
    }

    fn set_chat_menu_button(&self, params: &SetChatMenuButton) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setChatMenuButton").json(params))
    }

    fn get_chat_menu_button(&self, params: &GetChatMenuButton) -> Result<MenuButtons, Error> {
        self.respond_with::<MenuButtons>(self.request("getChatMenuButton").json(params))
    }

    fn set_my_default_administrator_rights(
        &self,
        params: &SetMyDefaultAdministratorRights,
    ) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setMyDefaultAdministratorRights").json(params))
    }

    fn get_my_default_administrator_rights(
        &self,
        params: &GetMyDefaultAdministratorRights,
    ) -> Result<ChatAdministratorRights, Error> {
        self.respond_with::<ChatAdministratorRights>(
            self.request("getMyDefaultAdministratorRights").json(params),
        )
    }

    fn edit_message_text(&self, params: &EditMessageText) -> Result<MessageResult, Error> {
        self.respond_with::<MessageResult>(self.request("editMessageText").json(params))
    }

    fn edit_message_caption(&self, params: &EditMessageCaption) -> Result<MessageResult, Error> {
        self.respond_with::<MessageResult>(self.request("editMessageCaption").json(params))
    }

    fn edit_message_media(&self, params: &EditMessageMedia) -> Result<MessageResult, Error> {
        self.respond_with::<MessageResult>(self.request("editMessageMedia").json(params))
    }

    fn edit_message_live_location(
        &self,
        params: &EditMessageLiveLocation,
    ) -> Result<MessageResult, Error> {
        self.respond_with::<MessageResult>(self.request("editMessageLiveLocation").json(params))
    }

    fn stop_message_live_location(
        &self,
        params: &StopMessageLiveLocation,
    ) -> Result<MessageResult, Error> {
        self.respond_with::<MessageResult>(self.request("stopMessageLiveLocation").json(params))
    }

    fn edit_message_reply_markup(
        &self,
        params: &EditMessageReplyMarkup,
    ) -> Result<MessageResult, Error> {
        self.respond_with::<MessageResult>(self.request("editMessageReplyMarkup").json(params))
    }

    fn stop_poll(&self, params: &StopPoll) -> Result<Poll, Error> {
        self.respond_with::<Poll>(self.request("stopPoll").json(params))
    }

    fn delete_message(&self, params: &DeleteMessage) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("deleteMessage").json(params))
    }

    fn delete_messages(&self, params: &DeleteMessages) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("deleteMessages").json(params))
    }

    fn send_sticker(&self, params: &SendSticker) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendSticker").json(params))
    }

    fn get_sticker_set(&self, params: &GetStickerSet) -> Result<StickerSet, Error> {
        self.respond_with::<StickerSet>(self.request("getStickerSet").json(params))
    }

    fn get_custom_emoji_stickers(&self, params: &GetCustomEmojiStickers) -> Result<Sticker, Error> {
        self.respond_with::<Sticker>(self.request("getCustomEmojiStickers").json(params))
    }

    fn upload_sticker_file(&self, params: &UploadStickerFile) -> Result<File, Error> {
        self.respond_with::<File>(self.request("uploadStickerFile").json(params))
    }

    fn create_new_sticker_set(&self, params: &CreateNewStickerSet) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("createNewStickerSet").json(params))
    }

    fn add_sticker_to_set(&self, params: &AddStickerToSet) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("addStickerToSet").json(params))
    }

    fn set_sticker_position_in_set(&self, params: &SetStickerPositionInSet) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setStickerPositionInSet").json(params))
    }

    fn delete_sticker_from_set(&self, params: &DeleteStickerFromSet) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("deleteStickerFromSet").json(params))
    }

    fn set_sticker_emoji_list(&self, params: &SetStickerEmojiList) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setStickerEmojiList").json(params))
    }

    fn set_sticker_keywords(&self, params: &SetStickerKeywords) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setStickerKeywords").json(params))
    }

    fn set_sticker_mask_position(&self, params: &SetStickerMaskPosition) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setStickerMaskPosition").json(params))
    }

    fn set_sticker_set_title(&self, params: &SetStickerSetTitle) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setStickerSetTitle").json(params))
    }
    fn set_sticker_set_thumbnail(&self, params: &SetStickerSetThumbnail) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setStickerSetThumbnail").json(params))
    }

    fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        params: &SetCustomEmojiStickerSetThumbnail,
    ) -> Result<bool, Error> {
        self.respond_with::<bool>(
            self.request("setCustomEmojiStickerSetThumbnail")
                .json(params),
        )
    }

    fn delete_sticker_set(&self, params: &DeleteStickerSet) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("deleteStickerSet").json(params))
    }

    fn answer_inline_query(&self, params: &AnswerInlineQuery) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("answerInlineQuery").json(params))
    }

    fn answer_web_app_query(&self, params: &AnswerWebAppQuery) -> Result<SentWebAppMessage, Error> {
        self.respond_with::<SentWebAppMessage>(self.request("answerWebAppQuery").json(params))
    }

    fn send_invoice(&self, params: &SendInvoice) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendInvoice").json(params))
    }

    fn create_invoice_link(&self, params: &CreateInvoiceLink) -> Result<String, Error> {
        self.respond_with::<String>(self.request("createInvoiceLink").json(params))
    }

    fn answer_shipping_query(&self, params: &AnswerShippingQuery) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("answerShippingQuery").json(params))
    }

    fn answer_pre_checkout_query(&self, params: &AnswerPreCheckoutQuery) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("answerPreCheckoutQuery").json(params))
    }

    fn set_passport_data_errors(&self, params: &SetPassportDataErrors) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request("setPassportDataErrors").json(params))
    }

    fn send_game(&self, params: &SendGame) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request("sendGame").json(params))
    }

    fn set_game_score(&self, params: &SetGameScore) -> Result<MessageResult, Error> {
        self.respond_with::<MessageResult>(self.request("setGameScore").json(params))
    }

    fn get_game_high_scores(&self, params: &GetGameHighScores) -> Result<GameHighScore, Error> {
        self.respond_with::<GameHighScore>(self.request("getGameHighScores").json(params))
    }
}
