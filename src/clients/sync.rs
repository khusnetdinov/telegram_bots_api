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
use crate::api::params::unhide_general_forum_topic::UnhideGeneralForumTopic;
use crate::api::params::unpin_all_chat_messages::UnpinAllChatMessages;
use crate::api::params::unpin_all_forum_topic_messages::UnpinAllForumTopicMessages;
use crate::api::params::unpin_all_general_forum_topic_messages::UnpinAllGeneralForumTopicMessages;
use crate::api::params::upload_sticker_file::UploadStickerFile;
use crate::api::requests::sync::Requests;
use crate::api::responses::error::ResponseError;
use crate::api::responses::result::ResponseResult;
use crate::api::types::bot_command::BotCommand;
use crate::api::types::bot_description::BotDescription;
use crate::api::types::bot_name::BotName;
use crate::api::types::bot_short_description::BotShortDescription;
use crate::api::types::chat::Chat;
use crate::api::types::chat_administrator_rights::ChatAdministratorRights;
use crate::api::types::chat_invite_link::ChatInviteLink;
use crate::api::types::file::File;
use crate::api::types::forum_topic::ForumTopic;
use crate::api::types::message::Message;
use crate::api::types::message_id::MessageId;
use crate::api::types::poll::Poll;
use crate::api::types::sent_web_app_message::SentWebAppMessage;
use crate::api::types::sticker::Sticker;
use crate::api::types::sticker_set::StickerSet;
use crate::api::types::update::Update;
use crate::api::types::user::User;
use crate::api::types::user_chat_boosts::UserChatBoosts;
use crate::api::types::user_profile_photos::UserProfilePhotos;
use crate::api::types::webhook_info::WebhookInfo;
use crate::clients::traits::{Decoder, Requester, Responder};
use crate::config::Config;
use crate::errors::Error;
use reqwest::blocking::{ClientBuilder, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use std::time::Duration;

#[derive(Debug)]
pub struct Sync {
    client: reqwest::blocking::Client,
    offset: i64,
    limit: i64,
    timeout: u64,
    url: String,
}

impl Sync {
    pub fn new(config: &Config) -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .build()
            .unwrap();

        let offset = config.updates_offset;
        let limit = config.updates_limit;
        let timeout = config.updates_timeout;
        let url = config.build_url();

        Self {
            client,
            offset,
            limit,
            timeout,
            url,
        }
    }
}

impl Decoder for Sync {
    fn decode<T: DeserializeOwned>(&self, response: Response) -> Result<T, Error> {
        match serde_json::from_str::<ResponseResult<T>>(&response.text().unwrap()) {
            Ok(success) => Ok(success.result),
            Err(error) => Err(Error::Decode(error)),
        }
    }
}

impl Requester for Sync {
    fn request_for(&self, method: &str) -> RequestBuilder {
        self.client.post(format!("{}{}", self.url, method))
    }
}

impl Responder for Sync {
    fn respond_with<T: DeserializeOwned>(
        &self,
        response: Result<Response, reqwest::Error>,
    ) -> Result<T, Error> {
        match response {
            Ok(response) => match response.status().as_u16() {
                200 => self.decode::<T>(response),
                _ => Err(Error::Response(ResponseError::new(
                    &response.text().unwrap(),
                ))),
            },
            Err(error) => Err(Error::Request(error)),
        }
    }
}

impl Requests for Sync {
    fn get_updates(&self, params: &GetUpdate) -> Result<Vec<Update>, Error> {
        self.respond_with::<Vec<Update>>(self.request_for("getUpdates").json(params).send())
    }

    fn set_webhook(&self, params: &SetWebhook) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request_for("setWebhook").json(params).send())
    }

    fn delete_webhook(&self, params: &DeleteWebhook) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request_for("deleteWebhook").json(params).send())
    }

    fn get_webhook_info(&self) -> Result<WebhookInfo, Error> {
        self.respond_with::<WebhookInfo>(self.request_for("getWebhookInfo").json(&{}).send())
    }

    fn get_me(&self) -> Result<User, Error> {
        self.respond_with::<User>(self.request_for("getMe").json(&{}).send())
    }

    fn log_out(&self) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request_for("logOut").json(&{}).send())
    }

    fn close(&self) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request_for("close").json(&{}).send())
    }

    fn send_message(&self, params: &SendMessage) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request_for("sendMessage").json(params).send())
    }

    fn forward_message(&self, params: &ForwardMessage) -> Result<MessageId, Error> {
        self.respond_with::<MessageId>(self.request_for("forwardMessage").json(params).send())
    }

    fn forward_messages(&self, params: &ForwardMessages) -> Result<Vec<MessageId>, Error> {
        self.respond_with::<Vec<MessageId>>(self.request_for("forwardMessages").json(params).send())
    }

    fn copy_message(&self, params: &CopyMessage) -> Result<MessageId, Error> {
        self.respond_with::<MessageId>(self.request_for("copyMessage").json(params).send())
    }

    fn copy_messages(&self, params: &CopyMessages) -> Result<Vec<MessageId>, Error> {
        self.respond_with::<Vec<MessageId>>(self.request_for("copyMessages").json(params).send())
    }

    fn send_photo(&self, params: &SendPhoto) -> Result<Message, Error> {
        todo!()
    }

    fn send_audio(&self, params: &SendAudio) -> Result<Message, Error> {
        todo!()
    }

    fn send_document(&self, params: &SendDocument) -> Result<Message, Error> {
        todo!()
    }

    fn send_video(&self, params: &SendVideo) -> Result<Message, Error> {
        todo!()
    }

    fn send_animation(&self, params: &SendAnimation) -> Result<Message, Error> {
        todo!()
    }

    fn send_voice(&self, params: &SendVoice) -> Result<Message, Error> {
        todo!()
    }

    fn send_video_note(&self, params: &SendVideoNote) -> Result<Message, Error> {
        todo!()
    }

    fn send_media_group(&self, params: &SendMediaGroup) -> Result<Message, Error> {
        todo!()
    }

    fn send_location(&self, params: &SendLocation) -> Result<Message, Error> {
        todo!()
    }

    fn send_venue(&self, params: &SendVenue) -> Result<Message, Error> {
        todo!()
    }

    fn send_contact(&self, params: &SendContact) -> Result<Message, Error> {
        todo!()
    }

    fn send_poll(&self, params: &SendPoll) -> Result<Message, Error> {
        todo!()
    }

    fn send_dice(&self, params: &SendDice) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request_for("sendDice").json(params).send())
    }

    fn send_chat_action(&self, params: &SendChatAction) -> Result<bool, Error> {
        todo!()
    }

    fn set_message_reaction(&self, params: &SetMessageReaction) -> Result<bool, Error> {
        todo!()
    }

    fn get_user_profile_photos(
        &self,
        params: &GetUserProfilePhotos,
    ) -> Result<UserProfilePhotos, Error> {
        todo!()
    }

    fn get_file(&self, params: GetFile) -> Result<File, Error> {
        todo!()
    }

    fn ban_chat_member(&self, params: &BanChatMember) -> Result<bool, Error> {
        todo!()
    }

    fn unban_chat_member(&self, params: &UnbanChatMember) -> Result<bool, Error> {
        todo!()
    }

    fn restrict_chat_member(&self, params: &RestrictChatMember) -> Result<bool, Error> {
        todo!()
    }

    fn promote_chat_member(&self, params: &PromoteChatMember) -> Result<bool, Error> {
        todo!()
    }

    fn set_chat_administrator_custom_title(
        &self,
        params: &SetChatAdministratorCustomTitle,
    ) -> Result<bool, Error> {
        todo!()
    }

    fn ban_chat_sender_chat(&self, params: &BanChatMember) -> Result<bool, Error> {
        todo!()
    }

    fn unban_chat_sender_chat(&self, params: &UnbanChatMember) -> Result<bool, Error> {
        todo!()
    }

    fn set_chat_permissions(&self, params: &SetChatPermissions) -> Result<bool, Error> {
        todo!()
    }

    fn export_chat_invite_link(&self, params: &ExportChatInviteLink) -> Result<String, Error> {
        todo!()
    }

    fn create_chat_invite_link(
        &self,
        params: &CreateChatInviteLink,
    ) -> Result<ChatInviteLink, Error> {
        todo!()
    }

    fn edit_chat_invite_link(&self, params: &EditChatInviteLink) -> Result<ChatInviteLink, Error> {
        todo!()
    }

    fn revoke_chat_invite_link(
        &self,
        params: &RevokeChatInviteLink,
    ) -> Result<ChatInviteLink, Error> {
        todo!()
    }

    fn approve_chat_join_request(&self, params: &ApproveChatJoinRequest) -> Result<bool, Error> {
        todo!()
    }

    fn decline_chat_join_request(&self, params: &DeclineChatJoinRequest) -> Result<bool, Error> {
        todo!()
    }

    fn set_chat_photo(&self, params: &SetChatPhoto) -> Result<bool, Error> {
        todo!()
    }

    fn delete_chat_photo(&self, params: &DeleteChatPhoto) -> Result<bool, Error> {
        todo!()
    }

    fn set_chat_title(&self, params: &SetChatTitle) -> Result<bool, Error> {
        todo!()
    }

    fn set_chat_description(&self, params: &SetChatDescription) -> Result<bool, Error> {
        todo!()
    }

    fn pin_chat_message(&self, params: &PinChatMessage) -> Result<bool, Error> {
        todo!()
    }

    fn unpin_chat_message(&self, params: &UnbanChatMember) -> Result<bool, Error> {
        todo!()
    }

    fn unpin_all_chat_messages(&self, params: &UnpinAllChatMessages) -> Result<bool, Error> {
        todo!()
    }

    fn leave_chat(&self, params: &LeaveChat) -> Result<bool, Error> {
        todo!()
    }

    fn get_chat(&self, params: &GetChat) -> Result<Chat, Error> {
        todo!()
    }

    fn get_chat_administrators(
        &self,
        params: &GetChatAdministrators,
    ) -> Result<Vec<ChatMember>, Error> {
        todo!()
    }

    fn get_chat_member_count(&self, params: &GetChatMemberCount) -> Result<i64, Error> {
        todo!()
    }

    fn get_chat_member(&self, params: &GetChatMember) -> Result<ChatMember, Error> {
        todo!()
    }

    fn set_chat_sticker_set(&self, params: &SetChatStickerSet) -> Result<bool, Error> {
        todo!()
    }

    fn delete_chat_sticker_set(&self, params: &DeleteChatStickerSet) -> Result<bool, Error> {
        todo!()
    }

    fn get_forum_topic_icon_stickers(
        &self,
        params: &GetForumTopicIconStickers,
    ) -> Result<Vec<Sticker>, Error> {
        todo!()
    }

    fn create_forum_topic(&self, params: &CreateForumTopic) -> Result<ForumTopic, Error> {
        todo!()
    }

    fn edit_forum_topic(&self, params: &EditForumTopic) -> Result<bool, Error> {
        todo!()
    }

    fn close_forum_topic(&self, params: &CloseForumTopic) -> Result<bool, Error> {
        todo!()
    }

    fn reopen_forum_topic(&self, params: &ReopenForumTopic) -> Result<bool, Error> {
        todo!()
    }

    fn delete_forum_topic(&self, params: &DeleteForumTopic) -> Result<bool, Error> {
        todo!()
    }

    fn unpin_all_forum_topic_messages(
        &self,
        params: &UnpinAllForumTopicMessages,
    ) -> Result<bool, Error> {
        todo!()
    }

    fn edit_general_forum_topic(&self, params: &EditGeneralForumTopic) -> Result<bool, Error> {
        todo!()
    }

    fn close_general_forum_topic(&self, params: &CloseGeneralForumTopic) -> Result<bool, Error> {
        todo!()
    }

    fn reopen_general_forum_topic(&self, params: &ReopenGeneralForumTopic) -> Result<bool, Error> {
        todo!()
    }

    fn hide_general_forum_topic(&self, params: &HideGeneralForumTopic) -> Result<bool, Error> {
        todo!()
    }

    fn unhide_general_forum_topic(&self, params: &UnhideGeneralForumTopic) -> Result<bool, Error> {
        todo!()
    }

    fn unpin_all_general_forum_topic_messages(
        &self,
        params: &UnpinAllGeneralForumTopicMessages,
    ) -> Result<bool, Error> {
        todo!()
    }

    fn answer_callback_query(&self, params: &AnswerCallbackQuery) -> Result<bool, Error> {
        todo!()
    }

    fn get_user_chat_boosts(&self, params: &GetUserChatBoosts) -> Result<UserChatBoosts, Error> {
        todo!()
    }

    fn set_my_commands(&self, params: &SetMyCommands) -> Result<bool, Error> {
        todo!()
    }

    fn delete_my_commands(&self, params: &DeleteMyCommands) -> Result<bool, Error> {
        todo!()
    }

    fn get_my_commands(&self, params: &GetMyCommands) -> Result<Vec<BotCommand>, Error> {
        todo!()
    }

    fn set_my_name(&self, params: &SetMyName) -> Result<bool, Error> {
        todo!()
    }

    fn get_my_name(&self, params: &GetMyName) -> Result<BotName, Error> {
        todo!()
    }

    fn set_my_description(&self, params: &SetMyDescription) -> Result<bool, Error> {
        todo!()
    }

    fn get_my_description(&self, params: &GetMyDescription) -> Result<BotDescription, Error> {
        todo!()
    }

    fn set_my_short_description(&self, params: &SetMyShortDescription) -> Result<bool, Error> {
        todo!()
    }

    fn get_my_short_description(
        &self,
        params: &GetMyShortDescription,
    ) -> Result<BotShortDescription, Error> {
        todo!()
    }

    fn set_chat_menu_button(&self, params: &SetChatMenuButton) -> Result<bool, Error> {
        todo!()
    }

    fn get_chat_menu_button(&self, params: &GetChatMenuButton) -> Result<MenuButton, Error> {
        todo!()
    }

    fn set_my_default_administrator_rights(
        &self,
        params: &SetMyDefaultAdministratorRights,
    ) -> Result<bool, Error> {
        todo!()
    }

    fn get_my_default_administrator_rights(
        &self,
        params: &GetMyDefaultAdministratorRights,
    ) -> Result<ChatAdministratorRights, Error> {
        todo!()
    }

    fn edit_message_text(&self, params: &EditMessageText) -> Result<MessageResult, Error> {
        todo!()
    }

    fn edit_message_caption(&self, params: &EditMessageCaption) -> Result<MessageResult, Error> {
        todo!()
    }

    fn edit_message_media(&self, params: &EditMessageMedia) -> Result<MessageResult, Error> {
        todo!()
    }

    fn edit_message_live_location(
        &self,
        params: &EditMessageLiveLocation,
    ) -> Result<MessageResult, Error> {
        todo!()
    }

    fn stop_message_live_location(
        &self,
        params: &StopMessageLiveLocation,
    ) -> Result<MessageResult, Error> {
        todo!()
    }

    fn edit_message_reply_markup(
        &self,
        params: &EditMessageReplyMarkup,
    ) -> Result<MessageResult, Error> {
        todo!()
    }

    fn stop_poll(&self, params: &StopPoll) -> Result<Poll, Error> {
        todo!()
    }

    fn delete_message(&self, params: &DeleteMessage) -> Result<bool, Error> {
        todo!()
    }

    fn delete_messages(&self, params: &DeleteMessages) -> Result<bool, Error> {
        todo!()
    }

    fn send_sticker(&self, params: &SendSticker) -> Result<Message, Error> {
        todo!()
    }

    fn get_sticker_set(&self, params: &GetStickerSet) -> Result<StickerSet, Error> {
        todo!()
    }

    fn get_custom_emoji_stickers(&self, params: &GetCustomEmojiStickers) -> Result<Sticker, Error> {
        todo!()
    }

    fn upload_sticker_file(&self, params: &UploadStickerFile) -> Result<File, Error> {
        todo!()
    }

    fn create_new_sticker_set(&self, params: &CreateNewStickerSet) -> Result<bool, Error> {
        todo!()
    }

    fn add_sticker_to_set(&self, params: &AddStickerToSet) -> Result<bool, Error> {
        todo!()
    }

    fn set_sticker_position_in_set(&self, params: &SetStickerPositionInSet) -> Result<bool, Error> {
        todo!()
    }

    fn delete_sticker_from_set(&self, params: &DeleteStickerFromSet) -> Result<bool, Error> {
        todo!()
    }

    fn set_sticker_emoji_list(&self, params: &SetStickerEmojiList) -> Result<bool, Error> {
        todo!()
    }

    fn set_sticker_keywords(&self, params: &SetStickerKeywords) -> Result<bool, Error> {
        todo!()
    }

    fn set_sticker_mask_position(&self, params: &SetStickerMaskPosition) -> Result<bool, Error> {
        todo!()
    }

    fn set_sticker_set_title(&self, params: &SetStickerSetTitle) -> Result<bool, Error> {
        todo!()
    }
    fn set_sticker_set_thumbnail(&self, params: &SetStickerSetThumbnail) -> Result<bool, Error> {
        todo!()
    }

    fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        params: &SetCustomEmojiStickerSetThumbnail,
    ) -> Result<bool, Error> {
        todo!()
    }

    fn delete_sticker_set(&self, params: &DeleteStickerSet) -> Result<bool, Error> {
        todo!()
    }

    fn answer_inline_query(&self, params: &AnswerInlineQuery) -> Result<bool, Error> {
        todo!()
    }

    fn answer_web_app_query(&self, params: &AnswerWebAppQuery) -> Result<SentWebAppMessage, Error> {
        todo!()
    }

    fn send_invoice(&self, params: &SendInvoice) -> Result<Message, Error> {
        todo!()
    }

    fn create_invoice_link(&self, params: &CreateInvoiceLink) -> Result<String, Error> {
        todo!()
    }

    fn answer_shipping_query(&self, params: &AnswerShippingQuery) -> Result<bool, Error> {
        todo!()
    }

    fn answer_pre_checkout_query(&self, params: &AnswerPreCheckoutQuery) -> Result<bool, Error> {
        todo!()
    }

    fn set_passport_data_errors(&self, params: &SetPassportDataErrors) -> Result<bool, Error> {
        todo!()
    }

    fn send_game(&self, params: &SendGame) -> Result<Message, Error> {
        todo!()
    }

    fn set_game_score(&self, params: &SetGameScore) -> Result<MessageResult, Error> {
        todo!()
    }

    fn get_game_high_scores(&self, params: &GetGameHighScores) -> Result<GetGameHighScores, Error> {
        todo!()
    }
}
