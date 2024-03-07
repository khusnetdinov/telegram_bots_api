use crate::api::enums::chat_member::ChatMember;
use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::file_input::FileInput;
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
use crate::api::requests::r#async::Requests;
use crate::api::types::bot_command::BotCommand;
use crate::api::types::bot_description::BotDescription;
use crate::api::types::bot_name::BotName;
use crate::api::types::bot_short_description::BotShortDescription;
use crate::api::types::chat::Chat;
use crate::api::types::chat_administrator_rights::ChatAdministratorRights;
use crate::api::types::chat_invite_link::ChatInviteLink;
use crate::api::types::file::File;
use crate::api::types::forum_topic::ForumTopic;
use crate::api::types::game_high_score::GameHighScore;
use crate::api::types::message::Message;
use crate::api::types::message_id::MessageId;
use crate::api::types::poll::Poll;
use crate::api::types::sent_web_app_message::SentWebAppMessage;
use crate::api::types::sticker::Sticker;
use crate::api::types::sticker_set::StickerSet;
use crate::api::types::update::Update;
use crate::api::types::user_chat_boosts::UserChatBoosts;
use crate::api::types::user_profile_photos::UserProfilePhotos;
use crate::api::types::webhook_info::WebhookInfo;
use crate::errors::Error;
use crate::tests::helpers::mocked_async::MockedAsync;
use std::fs;

#[tokio::test]
async fn get_updates_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/get_updates_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getUpdates", 200, &mock_response);

    let mock_result = mocked.result::<Vec<Update>>()?;
    let params = GetUpdate {
        limit: 100,
        offset: 249563340,
        timeout: 0,
        ..Default::default()
    };
    let real_result = mocked.client.get_updates(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_updates_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/get_updates_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getUpdates", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = GetUpdate {
        limit: 100,
        offset: 249563340,
        timeout: 0,
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.get_updates(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_webhook_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/set_webhook_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setWebhook", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetWebhook {
        url: String::from("https:78b3-91-202-26-13.ngrok-free.app"),
        ..Default::default()
    };
    let real_result = mocked.client.set_webhook(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_webhook_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/set_webhook_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setWebhook", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetWebhook {
        url: String::from("https:78b3-91-202-26-13.ngrok-free.app"),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.set_webhook(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn delete_webhook_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_webhook_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteWebhook", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = DeleteWebhook {
        ..Default::default()
    };
    let real_result = mocked.client.delete_webhook(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn delete_webhook_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_webhook_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteWebhook", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = DeleteWebhook {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.delete_webhook(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn get_webhook_info_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_webhook_info_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getWebhookInfo", 200, &mock_response);

    let mock_result = mocked.result::<WebhookInfo>()?;
    let real_result = mocked.client.get_webhook_info().await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_webhook_info_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_webhook_info_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getWebhookInfo", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    if let Error::Response(real_error) = mocked.client.get_webhook_info().await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

// #[tokio::test]
// async fn get_me_success() -> Result<(), Error> {
//     let mock_response = fs::read_to_string("src/tests/responses/get_me_success.json").unwrap();
//
//     let mut server = mockito::Server::new();
//     let mocked = MockedAsync::new(&mut server, "getMe", 200, &mock_response);
//
//     let mock_result = mocked.result::<User>()?;
//     let real_result = mocked.client.get_me().await?;
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
//
//     Ok(())
// }

// #[tokio::test]
// async fn get_me_error() -> Result<(), Error> {
//     let mock_response = fs::read_to_string("src/tests/responses/get_me_error.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = MockedAsync::new(&mut server, "getMe", 401, &mock_response);
//
//     let mock_error = mocked.result_error()?;
//     if let Error::Response(real_error) = mocked.client.get_me().await.unwrap_err()  {
//         assert_eq!(mock_error, real_error);
//         mocked.server.assert();
//     }
//
//     Ok(())
// }

#[tokio::test]
async fn log_out_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/log_out_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "logOut", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let real_result = mocked.client.log_out().await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn log_out_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/log_out_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "logOut", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    if let Error::Response(real_error) = mocked.client.log_out().await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn close_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/close_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "close", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let real_result = mocked.client.close().await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn close_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/close_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "close", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    if let Error::Response(real_error) = mocked.client.close().await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_message_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_message_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendMessage", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendMessage {
        chat_id: ChatUId::from(147951145),
        text: "Hello World".to_string(),
        ..Default::default()
    };
    let real_result = mocked.client.send_message(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_message_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_message_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendMessage", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendMessage {
        chat_id: ChatUId::from(147951145),
        text: "Hello World".to_string(),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_message(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn forward_message_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/forward_message_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "forwardMessage", 200, &mock_response);

    let mock_result = mocked.result::<MessageId>()?;
    let params = ForwardMessage {
        message_id: MessageId::from(456),
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    let real_result = mocked.client.forward_message(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn forward_message_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/forward_message_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "forwardMessage", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = ForwardMessage {
        message_id: MessageId::from(456),
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.forward_message(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn forward_messages_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/forward_messages_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "forwardMessages", 200, &mock_response);

    let mock_result = mocked.result::<Vec<MessageId>>()?;
    let params = ForwardMessages {
        message_ids: vec![MessageId::from(456)],
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    let real_result = mocked.client.forward_messages(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn forward_messages_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/forward_messages_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "forwardMessages", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = ForwardMessages {
        message_ids: vec![MessageId::from(455), MessageId::from(456)],
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.forward_messages(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn copy_message_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/copy_message_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "copyMessage", 200, &mock_response);

    let mock_result = mocked.result::<MessageId>()?;
    let params = CopyMessage {
        message_id: MessageId::from(456),
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    let real_result = mocked.client.copy_message(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn copy_message_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/copy_message_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "copyMessage", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = CopyMessage {
        message_id: MessageId::from(456),
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.copy_message(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn copy_messages_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/copy_messages_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "copyMessages", 200, &mock_response);

    let mock_result = mocked.result::<Vec<MessageId>>()?;
    let params = CopyMessages {
        message_ids: vec![MessageId::from(456)],
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    let real_result = mocked.client.copy_messages(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn copy_messages_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/copy_messages_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "copyMessages", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = CopyMessages {
        message_ids: vec![MessageId::from(455), MessageId::from(456)],
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.copy_messages(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_photo_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_photo_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendPhoto", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendPhoto {
        ..Default::default()
    };
    let real_result = mocked.client.send_photo(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_photo_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_photo_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendPhoto", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendPhoto {
        chat_id: ChatUId::from(147951145),
        photo: FileInput::String(String::from("")),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_photo(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_audio_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_audio_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendAudio", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendAudio {
        ..Default::default()
    };
    let real_result = mocked.client.send_audio(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_audio_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_audio_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendAudio", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendAudio {
        chat_id: ChatUId::from(147951145),
        audio: FileInput::String(String::from("")),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_audio(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_document_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_document_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendDocument", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendDocument {
        ..Default::default()
    };
    let real_result = mocked.client.send_document(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_document_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_document_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendDocument", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendDocument {
        chat_id: ChatUId::from(147951145),
        document: FileInput::String(String::from("")),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_document(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_video_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_video_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendVideo", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendVideo {
        ..Default::default()
    };
    let real_result = mocked.client.send_video(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_video_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_video_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendVideo", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendVideo {
        chat_id: ChatUId::from(147951145),
        video: FileInput::String(String::from("")),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_video(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_animation_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_animation_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendAnimation", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendAnimation {
        ..Default::default()
    };
    let real_result = mocked.client.send_animation(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_animation_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_animation_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendAnimation", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendAnimation {
        chat_id: ChatUId::from(147951145),
        animation: FileInput::String(String::from("")),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_animation(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_voice_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_voice_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendVoice", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendVoice {
        ..Default::default()
    };
    let real_result = mocked.client.send_voice(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_voice_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_voice_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendVoice", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendVoice {
        chat_id: ChatUId::from(147951145),
        voice: FileInput::String(String::from("")),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_voice(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_video_note_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_video_note_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendVideoNote", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendVideoNote {
        ..Default::default()
    };
    let real_result = mocked.client.send_video_note(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_video_note_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_video_note_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendVideoNote", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendVideoNote {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_video_note(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_media_group_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_media_group_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendMediaGroup", 200, &mock_response);

    let mock_result = mocked.result::<Vec<Message>>()?;
    let params = SendMediaGroup {
        ..Default::default()
    };
    let real_result = mocked.client.send_media_group(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_media_group_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_media_group_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendMediaGroup", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendMediaGroup {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_media_group(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_location_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_location_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendLocation", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendLocation {
        ..Default::default()
    };
    let real_result = mocked.client.send_location(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_location_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_location_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendLocation", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendLocation {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_location(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_venue_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_venue_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendVenue", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendVenue {
        ..Default::default()
    };
    let real_result = mocked.client.send_venue(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_venue_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_venue_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendVenue", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendVenue {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_venue(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

async fn send_contact_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_contact_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendContact", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendContact {
        ..Default::default()
    };
    let real_result = mocked.client.send_contact(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_contact_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_contact_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendContact", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendContact {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_contact(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

async fn send_poll_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_poll_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendPoll", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendPoll {
        ..Default::default()
    };
    let real_result = mocked.client.send_poll(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_poll_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_poll_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendPoll", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendPoll {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_poll(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_dice_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_dice_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendDice", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendDice {
        chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    let real_result = mocked.client.send_dice(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_dice_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_dice_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendDice", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendDice {
        chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_dice(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_chat_action_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_chat_action_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendChatAction", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SendChatAction {
        ..Default::default()
    };
    let real_result = mocked.client.send_chat_action(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_chat_action_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_chat_action_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendChatAction", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendChatAction {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_chat_action(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_message_reaction_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_message_reaction_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setMessageReaction", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetMessageReaction {
        ..Default::default()
    };
    let real_result = mocked.client.set_message_reaction(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_message_reaction_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_message_reaction_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setMessageReaction", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetMessageReaction {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_message_reaction(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn get_user_profile_photos_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_user_profile_photos_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getUserProfilePhotos", 200, &mock_response);

    let mock_result = mocked.result::<UserProfilePhotos>()?;
    let params = GetUserProfilePhotos {
        ..Default::default()
    };
    let real_result = mocked.client.get_user_profile_photos(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_user_profile_photos_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_user_profile_photos_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getUserProfilePhotos", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = GetUserProfilePhotos {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .get_user_profile_photos(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn get_file_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/get_file_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getFile", 200, &mock_response);

    let mock_result = mocked.result::<File>()?;
    let params = GetFile {
        ..Default::default()
    };
    let real_result = mocked.client.get_file(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_file_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/get_file_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getFile", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = GetFile {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.get_file(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn ban_chat_member_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/ban_chat_member_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "banChatMember", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = BanChatMember {
        ..Default::default()
    };
    let real_result = mocked.client.ban_chat_member(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn ban_chat_member_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/ban_chat_member_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "banChatMember", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = BanChatMember {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.ban_chat_member(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn unban_chat_member_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unban_chat_member_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "unbanChatMember", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = UnbanChatMember {
        ..Default::default()
    };
    let real_result = mocked.client.unban_chat_member(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn unban_chat_member_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unban_chat_member_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "unbanChatMember", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = UnbanChatMember {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.unban_chat_member(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn restrict_chat_member_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/restrict_chat_member_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "restrictChatMember", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = RestrictChatMember {
        ..Default::default()
    };
    let real_result = mocked.client.restrict_chat_member(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn restrict_chat_member_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/restrict_chat_member_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "restrictChatMember", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = RestrictChatMember {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .restrict_chat_member(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn promote_chat_member_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/promote_chat_member_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "promoteChatMember", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = PromoteChatMember {
        ..Default::default()
    };
    let real_result = mocked.client.promote_chat_member(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn promote_chat_member_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/promote_chat_member_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "promoteChatMember", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = PromoteChatMember {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .promote_chat_member(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_chat_administrator_custom_title_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_administrator_custom_title_success.json")
            .unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(
        &mut server,
        "setChatAdministratorCustomTitle",
        200,
        &mock_response,
    );

    let mock_result = mocked.result::<bool>()?;
    let params = SetChatAdministratorCustomTitle {
        ..Default::default()
    };
    let real_result = mocked
        .client
        .set_chat_administrator_custom_title(&params)
        .await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_chat_administrator_custom_title_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_administrator_custom_title_error.json")
            .unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(
        &mut server,
        "setChatAdministratorCustomTitle",
        400,
        &mock_response,
    );

    let mock_error = mocked.result_error()?;
    let params = SetChatAdministratorCustomTitle {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_chat_administrator_custom_title(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn ban_chat_sender_chat_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/ban_chat_sender_chat_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "banChatSenderChat", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = BanChatSenderChat {
        ..Default::default()
    };
    let real_result = mocked.client.ban_chat_sender_chat(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn ban_chat_sender_chat_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/ban_chat_sender_chat_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "banChatSenderChat", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = BanChatSenderChat {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .ban_chat_sender_chat(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn unban_chat_sender_chat_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unban_chat_sender_chat_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "unbanChatSenderChat", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = UnbanChatSenderChat {
        ..Default::default()
    };
    let real_result = mocked.client.unban_chat_sender_chat(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn unban_chat_sender_chat_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unban_chat_sender_chat_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "unbanChatSenderChat", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = UnbanChatSenderChat {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .unban_chat_sender_chat(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_chat_permissions_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_permissions_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setChatPermissions", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetChatPermissions {
        ..Default::default()
    };
    let real_result = mocked.client.set_chat_permissions(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_chat_permissions_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_permissions_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setChatPermissions", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetChatPermissions {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_chat_permissions(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn export_chat_invite_link_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/export_chat_invite_link_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "exportChatInviteLink", 200, &mock_response);

    let mock_result = mocked.result::<String>()?;
    let params = ExportChatInviteLink {
        ..Default::default()
    };
    let real_result = mocked.client.export_chat_invite_link(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn export_chat_invite_link_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/export_chat_invite_link_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "exportChatInviteLink", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = ExportChatInviteLink {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .export_chat_invite_link(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn create_chat_invite_link_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/create_chat_invite_link_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "createChatInviteLink", 200, &mock_response);

    let mock_result = mocked.result::<ChatInviteLink>()?;
    let params = CreateChatInviteLink {
        ..Default::default()
    };
    let real_result = mocked.client.create_chat_invite_link(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn create_chat_invite_link_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/create_chat_invite_link_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "createChatInviteLink", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = CreateChatInviteLink {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .create_chat_invite_link(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn edit_chat_invite_link_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_chat_invite_link_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editChatInviteLink", 200, &mock_response);

    let mock_result = mocked.result::<ChatInviteLink>()?;
    let params = EditChatInviteLink {
        ..Default::default()
    };
    let real_result = mocked.client.edit_chat_invite_link(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn edit_chat_invite_link_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_chat_invite_link_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editChatInviteLink", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = EditChatInviteLink {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .edit_chat_invite_link(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn revoke_chat_invite_link_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/revoke_chat_invite_link_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "revokeChatInviteLink", 200, &mock_response);

    let mock_result = mocked.result::<ChatInviteLink>()?;
    let params = RevokeChatInviteLink {
        ..Default::default()
    };
    let real_result = mocked.client.revoke_chat_invite_link(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn revoke_chat_invite_link_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/revoke_chat_invite_link_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "revokeChatInviteLink", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = RevokeChatInviteLink {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .revoke_chat_invite_link(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn approve_chat_join_request_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/approve_chat_join_request_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "approveChatJoinRequest", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = ApproveChatJoinRequest {
        ..Default::default()
    };
    let real_result = mocked.client.approve_chat_join_request(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn approve_chat_join_request_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/approve_chat_join_request_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "approveChatJoinRequest", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = ApproveChatJoinRequest {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .approve_chat_join_request(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn decline_chat_join_request_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/decline_chat_join_request_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "declineChatJoinRequest", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = DeclineChatJoinRequest {
        ..Default::default()
    };
    let real_result = mocked.client.decline_chat_join_request(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn decline_chat_join_request_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/decline_chat_join_request_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "declineChatJoinRequest", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = DeclineChatJoinRequest {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .decline_chat_join_request(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_chat_photo_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_photo_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setChatPhoto", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetChatPhoto {
        ..Default::default()
    };
    let real_result = mocked.client.set_chat_photo(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_chat_photo_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_photo_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setChatPhoto", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetChatPhoto {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.set_chat_photo(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn delete_chat_photo_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_chat_photo_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteChatPhoto", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = DeleteChatPhoto {
        ..Default::default()
    };
    let real_result = mocked.client.delete_chat_photo(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn delete_chat_photo_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_chat_photo_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteChatPhoto", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = DeleteChatPhoto {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.delete_chat_photo(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_chat_title_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_title_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setChatTitle", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetChatTitle {
        ..Default::default()
    };
    let real_result = mocked.client.set_chat_title(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_chat_title_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_title_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setChatTitle", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetChatTitle {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.set_chat_title(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_chat_description_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_description_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setChatDescription", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetChatDescription {
        ..Default::default()
    };
    let real_result = mocked.client.set_chat_description(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_chat_description_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_description_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setChatDescription", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetChatDescription {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_chat_description(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn pin_chat_message_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/pin_chat_message_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "pinChatMessage", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = PinChatMessage {
        ..Default::default()
    };
    let real_result = mocked.client.pin_chat_message(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn pin_chat_message_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/pin_chat_message_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "pinChatMessage", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = PinChatMessage {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.pin_chat_message(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn unpin_chat_message_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unpin_chat_message_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "unpinChatMessage", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = UnpinChatMessage {
        ..Default::default()
    };
    let real_result = mocked.client.unpin_chat_message(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn unpin_chat_message_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unpin_chat_message_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "unpinChatMessage", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = UnpinChatMessage {
        ..Default::default()
    };
    if let Error::Response(real_error) =
        mocked.client.unpin_chat_message(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn unpin_all_chat_messages_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unpin_all_chat_messages_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "unpinAllChatMessages", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = UnpinAllChatMessages {
        ..Default::default()
    };
    let real_result = mocked.client.unpin_all_chat_messages(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn unpin_all_chat_messages_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unpin_all_chat_messages_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "unpinAllChatMessages", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = UnpinAllChatMessages {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .unpin_all_chat_messages(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn leave_chat_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/leave_chat_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "leaveChat", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = LeaveChat {
        ..Default::default()
    };
    let real_result = mocked.client.leave_chat(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn leave_chat_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/leave_chat_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "leaveChat", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = LeaveChat {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.leave_chat(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn get_chat_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/get_chat_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getChat", 200, &mock_response);

    let mock_result = mocked.result::<Chat>()?;
    let params = GetChat {
        ..Default::default()
    };
    let real_result = mocked.client.get_chat(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_chat_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/get_chat_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getChat", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = GetChat {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.get_chat(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn get_chat_administrators_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_chat_administrators_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getChatAdministrators", 200, &mock_response);

    let mock_result = mocked.result::<Vec<ChatMember>>()?;
    let params = GetChatAdministrators {
        ..Default::default()
    };
    let real_result = mocked.client.get_chat_administrators(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_chat_administrators_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_chat_administrators_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getChatAdministrators", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = GetChatAdministrators {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .get_chat_administrators(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn get_chat_member_count_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_chat_member_count_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getChatMemberCount", 200, &mock_response);

    let mock_result = mocked.result::<i64>()?;
    let params = GetChatMemberCount {
        ..Default::default()
    };
    let real_result = mocked.client.get_chat_member_count(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_chat_member_count_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_chat_member_count_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getChatMemberCount", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = GetChatMemberCount {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .get_chat_member_count(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn get_chat_member_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_chat_member_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getChatMember", 200, &mock_response);

    let mock_result = mocked.result::<ChatMember>()?;
    let params = GetChatMember {
        ..Default::default()
    };
    let real_result = mocked.client.get_chat_member(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_chat_member_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_chat_member_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getChatMember", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = GetChatMember {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.get_chat_member(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_chat_sticker_set_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_sticker_set_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setChatStickerSet", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetChatStickerSet {
        ..Default::default()
    };
    let real_result = mocked.client.set_chat_sticker_set(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_chat_sticker_set_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_sticker_set_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setChatStickerSet", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetChatStickerSet {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_chat_sticker_set(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn delete_chat_sticker_set_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_chat_sticker_set_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteChatStickerSet", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = DeleteChatStickerSet {
        ..Default::default()
    };
    let real_result = mocked.client.delete_chat_sticker_set(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn delete_chat_sticker_set_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_chat_sticker_set_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteChatStickerSet", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = DeleteChatStickerSet {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .delete_chat_sticker_set(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn get_forum_topic_icon_stickers_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_forum_topic_icon_stickers.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(
        &mut server,
        "getForumTopicIconStickers",
        200,
        &mock_response,
    );

    let mock_result = mocked.result::<Vec<Sticker>>()?;
    let params = GetForumTopicIconStickers {
        ..Default::default()
    };
    let real_result = mocked.client.get_forum_topic_icon_stickers(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn create_forum_topic_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/create_forum_topic_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "createForumTopic", 200, &mock_response);

    let mock_result = mocked.result::<ForumTopic>()?;
    let params = CreateForumTopic {
        ..Default::default()
    };
    let real_result = mocked.client.create_forum_topic(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn create_forum_topic_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/create_forum_topic_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "createForumTopic", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = CreateForumTopic {
        ..Default::default()
    };
    if let Error::Response(real_error) =
        mocked.client.create_forum_topic(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn edit_forum_topic_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_forum_topic_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editForumTopic", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = EditForumTopic {
        ..Default::default()
    };
    let real_result = mocked.client.edit_forum_topic(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn edit_forum_topic_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_forum_topic_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editForumTopic", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = EditForumTopic {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.edit_forum_topic(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn close_forum_topic_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/close_forum_topic_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "closeForumTopic", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = CloseForumTopic {
        ..Default::default()
    };
    let real_result = mocked.client.close_forum_topic(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn close_forum_topic_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/close_forum_topic_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "closeForumTopic", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = CloseForumTopic {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.close_forum_topic(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn reopen_forum_topic_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/reopen_forum_topic_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "reopenForumTopic", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = ReopenForumTopic {
        ..Default::default()
    };
    let real_result = mocked.client.reopen_forum_topic(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn reopen_forum_topic_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/reopen_forum_topic_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "reopenForumTopic", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = ReopenForumTopic {
        ..Default::default()
    };
    if let Error::Response(real_error) =
        mocked.client.reopen_forum_topic(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn delete_forum_topic_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_forum_topic_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteForumTopic", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = DeleteForumTopic {
        ..Default::default()
    };
    let real_result = mocked.client.delete_forum_topic(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn delete_forum_topic_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_forum_topic_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteForumTopic", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = DeleteForumTopic {
        ..Default::default()
    };
    if let Error::Response(real_error) =
        mocked.client.delete_forum_topic(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn unpin_all_forum_topic_messages_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unpin_all_forum_topic_messages_success.json")
            .unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(
        &mut server,
        "unpinAllForumTopicMessages",
        200,
        &mock_response,
    );

    let mock_result = mocked.result::<bool>()?;
    let params = UnpinAllForumTopicMessages {
        ..Default::default()
    };
    let real_result = mocked
        .client
        .unpin_all_forum_topic_messages(&params)
        .await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn unpin_all_forum_topic_messages_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unpin_all_forum_topic_messages_error.json")
            .unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(
        &mut server,
        "unpinAllForumTopicMessages",
        200,
        &mock_response,
    );

    let mock_error = mocked.result_error()?;
    let params = UnpinAllForumTopicMessages {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .unpin_all_forum_topic_messages(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn edit_general_forum_topic_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_general_forum_topic_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editGeneralForumTopic", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = EditGeneralForumTopic {
        ..Default::default()
    };
    let real_result = mocked.client.edit_general_forum_topic(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn edit_general_forum_topic_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_general_forum_topic_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editGeneralForumTopic", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = EditGeneralForumTopic {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .edit_general_forum_topic(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}
#[tokio::test]
async fn close_general_forum_topic_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/close_general_forum_topic_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "closeGeneralForumTopic", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = CloseGeneralForumTopic {
        ..Default::default()
    };
    let real_result = mocked.client.close_general_forum_topic(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn close_general_forum_topic_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/close_general_forum_topic_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "closeGeneralForumTopic", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = CloseGeneralForumTopic {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .close_general_forum_topic(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn reopen_general_forum_topic_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/reopen_general_forum_topic_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "reopenGeneralForumTopic", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = ReopenGeneralForumTopic {
        ..Default::default()
    };
    let real_result = mocked.client.reopen_general_forum_topic(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn reopen_general_forum_topic_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/reopen_general_forum_topic_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "reopenGeneralForumTopic", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = ReopenGeneralForumTopic {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .reopen_general_forum_topic(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn hide_general_forum_topic_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/hide_general_forum_topic_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "hideGeneralForumTopic", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = HideGeneralForumTopic {
        ..Default::default()
    };
    let real_result = mocked.client.hide_general_forum_topic(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn hide_general_forum_topic_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/hide_general_forum_topic_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "hideGeneralForumTopic", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = HideGeneralForumTopic {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .hide_general_forum_topic(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn unhide_general_forum_topic_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unhide_general_forum_topic_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "unhideGeneralForumTopic", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = UnhideGeneralForumTopic {
        ..Default::default()
    };
    let real_result = mocked.client.unhide_general_forum_topic(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn unhide_general_forum_topic_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unhide_general_forum_topic_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "unhideGeneralForumTopic", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = UnhideGeneralForumTopic {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .unhide_general_forum_topic(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn unpin_all_general_forum_topic_messages_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string(
        "src/tests/responses/unpin_all_general_forum_topic_messages_success.json",
    )
    .unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(
        &mut server,
        "unpinAllGeneralForumTopicMessages",
        200,
        &mock_response,
    );

    let mock_result = mocked.result::<bool>()?;
    let params = UnpinAllGeneralForumTopicMessages {
        ..Default::default()
    };
    let real_result = mocked
        .client
        .unpin_all_general_forum_topic_messages(&params)
        .await
        .unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn unpin_all_general_forum_topic_messages_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/unpin_all_general_forum_topic_messages_error.json")
            .unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(
        &mut server,
        "unpinAllGeneralForumTopicMessages",
        200,
        &mock_response,
    );

    let mock_error = mocked.result_error()?;
    let params = UnpinAllGeneralForumTopicMessages {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .unpin_all_general_forum_topic_messages(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn answer_callback_query_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/answer_callback_query_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "answerCallbackQuery", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = AnswerCallbackQuery {
        ..Default::default()
    };
    let real_result = mocked.client.answer_callback_query(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn answer_callback_query_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/answer_callback_query_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "answerCallbackQuery", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = AnswerCallbackQuery {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .answer_callback_query(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn get_user_chat_boosts_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_user_chat_boosts_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getUserChatBoosts", 200, &mock_response);

    let mock_result = mocked.result::<UserChatBoosts>()?;
    let params = GetUserChatBoosts {
        ..Default::default()
    };
    let real_result = mocked.client.get_user_chat_boosts(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_user_chat_boosts_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_user_chat_boosts_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getUserChatBoosts", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = GetUserChatBoosts {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .get_user_chat_boosts(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_my_commands_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/set_my_commands.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setMyCommands", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetMyCommands {
        ..Default::default()
    };
    let real_result = mocked.client.set_my_commands(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn delete_my_commands_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/delete_my_commands.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteMyCommands", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = DeleteMyCommands {
        ..Default::default()
    };
    let real_result = mocked.client.delete_my_commands(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_my_commands_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/get_my_commands.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getMyCommands", 200, &mock_response);

    let mock_result = mocked.result::<Vec<BotCommand>>()?;
    let params = GetMyCommands {
        ..Default::default()
    };
    let real_result = mocked.client.get_my_commands(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_my_name_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/set_my_name_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setMyName", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetMyName {
        ..Default::default()
    };
    let real_result = mocked.client.set_my_name(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_my_name_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/set_my_name_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setMyName", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetMyName {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.set_my_name(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn get_my_name_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/get_my_name_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getMyName", 200, &mock_response);

    let mock_result = mocked.result::<BotName>()?;
    let params = GetMyName {
        ..Default::default()
    };
    let real_result = mocked.client.get_my_name(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_my_description_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/set_my_description.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setMyDescription", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetMyDescription {
        ..Default::default()
    };
    let real_result = mocked.client.set_my_description(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_my_description_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/get_my_description.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getMyDescription", 200, &mock_response);

    let mock_result = mocked.result::<BotDescription>()?;
    let params = GetMyDescription {
        ..Default::default()
    };
    let real_result = mocked.client.get_my_description(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_my_short_description_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_my_short_description.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setMyShortDescription", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetMyShortDescription {
        ..Default::default()
    };
    let real_result = mocked.client.set_my_short_description(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_my_short_description_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_my_short_description.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getMyShortDescription", 200, &mock_response);

    let mock_result = mocked.result::<BotShortDescription>()?;
    let params = GetMyShortDescription {
        ..Default::default()
    };
    let real_result = mocked.client.get_my_short_description(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_chat_menu_button_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_menu_button.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setChatMenuButton", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetChatMenuButton {
        ..Default::default()
    };
    let real_result = mocked.client.set_chat_menu_button(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_chat_menu_button_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_chat_menu_button.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getChatMenuButton", 200, &mock_response);

    let mock_result = mocked.result::<MenuButton>()?;
    let params = GetChatMenuButton {
        ..Default::default()
    };
    let real_result = mocked.client.get_chat_menu_button(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_my_default_administrator_rights_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_my_default_administrator_rights.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(
        &mut server,
        "setMyDefaultAdministratorRights",
        200,
        &mock_response,
    );

    let mock_result = mocked.result::<bool>()?;
    let params = SetMyDefaultAdministratorRights {
        ..Default::default()
    };
    let real_result = mocked
        .client
        .set_my_default_administrator_rights(&params)
        .await
        .unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_my_default_administrator_rights_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_my_default_administrator_rights.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(
        &mut server,
        "getMyDefaultAdministratorRights",
        200,
        &mock_response,
    );

    let mock_result = mocked.result::<ChatAdministratorRights>()?;
    let params = GetMyDefaultAdministratorRights {
        ..Default::default()
    };
    let real_result = mocked
        .client
        .get_my_default_administrator_rights(&params)
        .await
        .unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn edit_message_text_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_message_text_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editMessageText", 200, &mock_response);

    let mock_result = mocked.result::<MessageResult>()?;
    let params = EditMessageText {
        ..Default::default()
    };
    let real_result = mocked.client.edit_message_text(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

// #[tokio::test]
async fn edit_message_text_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_message_text_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editMessageText", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = EditMessageText {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.edit_message_text(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn edit_message_caption_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_message_caption_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editMessageCaption", 200, &mock_response);

    let mock_result = mocked.result::<MessageResult>()?;
    let params = EditMessageCaption {
        ..Default::default()
    };
    let real_result = mocked.client.edit_message_caption(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn edit_message_caption_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_message_caption_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editMessageCaption", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = EditMessageCaption {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .edit_message_caption(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn edit_message_media_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_message_media_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editMessageMedia", 200, &mock_response);

    let mock_result = mocked.result::<MessageResult>()?;
    let params = EditMessageMedia {
        ..Default::default()
    };
    let real_result = mocked.client.edit_message_media(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn edit_message_media_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_message_media_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editMessageMedia", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = EditMessageMedia {
        ..Default::default()
    };
    if let Error::Response(real_error) =
        mocked.client.edit_message_media(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn edit_message_live_location_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_message_live_location_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editMessageLiveLocation", 200, &mock_response);

    let mock_result = mocked.result::<MessageResult>()?;
    let params = EditMessageLiveLocation {
        ..Default::default()
    };
    let real_result = mocked.client.edit_message_live_location(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn edit_message_live_location_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_message_live_location_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editMessageLiveLocation", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = EditMessageLiveLocation {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .edit_message_live_location(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn stop_message_live_location_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/stop_message_live_location_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "stopMessageLiveLocation", 200, &mock_response);

    let mock_result = mocked.result::<MessageResult>()?;
    let params = StopMessageLiveLocation {
        ..Default::default()
    };
    let real_result = mocked.client.stop_message_live_location(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn stop_message_live_location_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/stop_message_live_location_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "stopMessageLiveLocation", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = StopMessageLiveLocation {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .stop_message_live_location(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

// #[tokio::test]
async fn edit_message_reply_markup_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_message_reply_markup_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editMessageReplyMarkup", 200, &mock_response);

    let mock_result = mocked.result::<MessageResult>()?;
    let params = EditMessageReplyMarkup {
        ..Default::default()
    };
    let real_result = mocked.client.edit_message_reply_markup(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn edit_message_reply_markup_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/edit_message_reply_markup_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "editMessageReplyMarkup", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = EditMessageReplyMarkup {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .edit_message_reply_markup(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn stop_poll_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/stop_poll_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "stopPoll", 200, &mock_response);

    let mock_result = mocked.result::<Poll>()?;
    let params = StopPoll {
        ..Default::default()
    };
    let real_result = mocked.client.stop_poll(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn stop_poll_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/stop_poll_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "stopPoll", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = StopPoll {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.stop_poll(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn delete_message_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_message_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteMessage", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = DeleteMessage {
        ..Default::default()
    };
    let real_result = mocked.client.delete_message(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn delete_message_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_message_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteMessage", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = DeleteMessage {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.delete_message(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn delete_messages_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_messages_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteMessages", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = DeleteMessages {
        ..Default::default()
    };
    let real_result = mocked.client.delete_messages(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn delete_messages_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_messages_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteMessages", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = DeleteMessages {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.delete_messages(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn send_sticker_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_sticker_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendSticker", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendSticker {
        ..Default::default()
    };
    let real_result = mocked.client.send_sticker(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_sticker_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_sticker_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendSticker", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendSticker {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_sticker(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn get_sticker_set_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_sticker_set_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getStickerSet", 200, &mock_response);

    let mock_result = mocked.result::<StickerSet>()?;
    let params = GetStickerSet {
        ..Default::default()
    };
    let real_result = mocked.client.get_sticker_set(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_sticker_set_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_sticker_set_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getStickerSet", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = GetStickerSet {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.get_sticker_set(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

// #[tokio::test]
async fn get_custom_emoji_stickers_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_custom_emoji_stickers_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getCustomEmojiStickers", 200, &mock_response);

    let mock_result = mocked.result::<Sticker>()?;
    let params = GetCustomEmojiStickers {
        ..Default::default()
    };
    let real_result = mocked.client.get_custom_emoji_stickers(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_custom_emoji_stickers_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_custom_emoji_stickers_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getCustomEmojiStickers", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = GetCustomEmojiStickers {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .get_custom_emoji_stickers(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

// #[tokio::test]
async fn upload_sticker_file_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/upload_sticker_file_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "uploadStickerFile", 200, &mock_response);

    let mock_result = mocked.result::<File>()?;
    let params = UploadStickerFile {
        ..Default::default()
    };
    let real_result = mocked.client.upload_sticker_file(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn upload_sticker_file_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/upload_sticker_file_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "uploadStickerFile", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = UploadStickerFile {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .upload_sticker_file(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

// #[tokio::test]
async fn create_new_sticker_set_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/create_new_sticker_set_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "createNewStickerSet", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = CreateNewStickerSet {
        ..Default::default()
    };
    let real_result = mocked.client.create_new_sticker_set(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn create_new_sticker_set_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/create_new_sticker_set_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "createNewStickerSet", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = CreateNewStickerSet {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .create_new_sticker_set(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

// #[tokio::test]
async fn add_sticker_to_set_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/add_sticker_to_set_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "addStickerToSet", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = AddStickerToSet {
        ..Default::default()
    };
    let real_result = mocked.client.add_sticker_to_set(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn add_sticker_to_set_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/add_sticker_to_set_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "addStickerToSet", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = AddStickerToSet {
        ..Default::default()
    };
    if let Error::Response(real_error) =
        mocked.client.add_sticker_to_set(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_sticker_position_in_set_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_sticker_position_in_set_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setStickerPositionInSet", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetStickerPositionInSet {
        ..Default::default()
    };
    let real_result = mocked.client.set_sticker_position_in_set(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_sticker_position_in_set_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_sticker_position_in_set_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setStickerPositionInSet", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetStickerPositionInSet {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_sticker_position_in_set(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn delete_sticker_from_set_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_sticker_from_set_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteStickerFromSet", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = DeleteStickerFromSet {
        ..Default::default()
    };
    let real_result = mocked.client.delete_sticker_from_set(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn delete_sticker_from_set_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_sticker_from_set_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteStickerFromSet", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = DeleteStickerFromSet {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .delete_sticker_from_set(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_sticker_emoji_list_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_sticker_emoji_list_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setStickerEmojiList", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetStickerEmojiList {
        ..Default::default()
    };
    let real_result = mocked.client.set_sticker_emoji_list(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_sticker_emoji_list_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_sticker_emoji_list_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setStickerEmojiList", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetStickerEmojiList {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_sticker_emoji_list(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_sticker_keywords_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_sticker_keywords_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setStickerKeywords", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetStickerKeywords {
        ..Default::default()
    };
    let real_result = mocked.client.set_sticker_keywords(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_sticker_keywords_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_sticker_keywords_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setStickerKeywords", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetStickerKeywords {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_sticker_keywords(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_sticker_mask_position_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_sticker_mask_position_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setStickerMaskPosition", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetStickerMaskPosition {
        ..Default::default()
    };
    let real_result = mocked.client.set_sticker_mask_position(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_sticker_mask_position_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_sticker_mask_position_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setStickerMaskPosition", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetStickerMaskPosition {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_sticker_mask_position(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_sticker_set_title_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_sticker_set_title_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setStickerSetTitle", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetStickerSetTitle {
        ..Default::default()
    };
    let real_result = mocked.client.set_sticker_set_title(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_sticker_set_title_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_sticker_set_title_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setStickerSetTitle", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetStickerSetTitle {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_sticker_set_title(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_sticker_set_thumbnail_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_sticker_set_thumbnail_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setStickerSetThumbnail", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetStickerSetThumbnail {
        ..Default::default()
    };
    let real_result = mocked.client.set_sticker_set_thumbnail(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_sticker_set_thumbnail_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_sticker_set_thumbnail_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setStickerSetThumbnail", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetStickerSetThumbnail {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_sticker_set_thumbnail(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_custom_emoji_sticker_set_thumbnail_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string(
        "src/tests/responses/set_custom_emoji_sticker_set_thumbnail_success.json",
    )
    .unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(
        &mut server,
        "setCustomEmojiStickerSetThumbnail",
        200,
        &mock_response,
    );

    let mock_result = mocked.result::<bool>()?;
    let params = SetCustomEmojiStickerSetThumbnail {
        ..Default::default()
    };
    let real_result = mocked
        .client
        .set_custom_emoji_sticker_set_thumbnail(&params)
        .await
        .unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_custom_emoji_sticker_set_thumbnail_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_custom_emoji_sticker_set_thumbnail_error.json")
            .unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(
        &mut server,
        "setCustomEmojiStickerSetThumbnail",
        400,
        &mock_response,
    );

    let mock_error = mocked.result_error()?;
    let params = SetCustomEmojiStickerSetThumbnail {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_custom_emoji_sticker_set_thumbnail(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn delete_sticker_set_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_sticker_set_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteStickerSet", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = DeleteStickerSet {
        ..Default::default()
    };
    let real_result = mocked.client.delete_sticker_set(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn delete_sticker_set_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_sticker_set_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "deleteStickerSet", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = DeleteStickerSet {
        ..Default::default()
    };
    if let Error::Response(real_error) =
        mocked.client.delete_sticker_set(&params).await.unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn answer_inline_query_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/answer_inline_query_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "answerInlineQuery", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = AnswerInlineQuery {
        ..Default::default()
    };
    let real_result = mocked.client.answer_inline_query(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn answer_inline_query_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/answer_inline_query_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "answerInlineQuery", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = AnswerInlineQuery {
        ..Default::default()
    };

    if let Error::Response(real_error) = mocked
        .client
        .answer_inline_query(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

// #[tokio::test]
async fn answer_web_app_query_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/answer_web_app_query_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "answerWebAppQuery", 200, &mock_response);

    let mock_result = mocked.result::<SentWebAppMessage>()?;
    let params = AnswerWebAppQuery {
        ..Default::default()
    };
    let real_result = mocked.client.answer_web_app_query(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn answer_web_app_query_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/answer_web_app_query_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "answerWebAppQuery", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = AnswerWebAppQuery {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .answer_web_app_query(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

// #[tokio::test]
async fn send_invoice_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_invoice_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendInvoice", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendInvoice {
        ..Default::default()
    };
    let real_result = mocked.client.send_invoice(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_invoice_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_invoice_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendInvoice", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendInvoice {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_invoice(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn create_invoice_link_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/create_invoice_link_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "createInvoiceLink", 200, &mock_response);

    let mock_result = mocked.result::<String>()?;
    let params = CreateInvoiceLink {
        ..Default::default()
    };
    let real_result = mocked.client.create_invoice_link(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn create_invoice_link_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/create_invoice_link_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "createInvoiceLink", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = CreateInvoiceLink {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .create_invoice_link(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn answer_shipping_query_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/answer_shipping_query_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "answerShippingQuery", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = AnswerShippingQuery {
        ..Default::default()
    };
    let real_result = mocked.client.answer_shipping_query(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn answer_shipping_query_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/answer_shipping_query_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "answerShippingQuery", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = AnswerShippingQuery {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .answer_shipping_query(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn answer_pre_checkout_query_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/answer_pre_checkout_query_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "answerPreCheckoutQuery", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = AnswerPreCheckoutQuery {
        ..Default::default()
    };
    let real_result = mocked.client.answer_pre_checkout_query(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn answer_pre_checkout_query_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/answer_pre_checkout_query_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "answerPreCheckoutQuery", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = AnswerPreCheckoutQuery {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .answer_pre_checkout_query(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

#[tokio::test]
async fn set_passport_data_errors_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_passport_data_errors_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setPassportDataErrors", 200, &mock_response);

    let mock_result = mocked.result::<bool>()?;
    let params = SetPassportDataErrors {
        ..Default::default()
    };
    let real_result = mocked.client.set_passport_data_errors(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_passport_data_errors_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_passport_data_errors_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setPassportDataErrors", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetPassportDataErrors {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .set_passport_data_errors(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

// #[tokio::test]
async fn send_game_success() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_game_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendGame", 200, &mock_response);

    let mock_result = mocked.result::<Message>()?;
    let params = SendGame {
        ..Default::default()
    };
    let real_result = mocked.client.send_game(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn send_game_error() -> Result<(), Error> {
    let mock_response = fs::read_to_string("src/tests/responses/send_game_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "sendGame", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SendGame {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.send_game(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

// #[tokio::test]
async fn set_game_score_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_game_score_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setGameScore", 200, &mock_response);

    let mock_result = mocked.result::<MessageResult>()?;
    let params = SetGameScore {
        ..Default::default()
    };
    let real_result = mocked.client.set_game_score(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn set_game_score_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_game_score_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setGameScore", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = SetGameScore {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.set_game_score(&params).await.unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}

// #[tokio::test]
async fn get_game_high_scores_success() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_game_high_scores_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "setGameScore", 200, &mock_response);

    let mock_result = mocked.result::<GameHighScore>()?;
    let params = GetGameHighScores {
        ..Default::default()
    };
    let real_result = mocked.client.get_game_high_scores(&params).await?;

    assert_eq!(mock_result, real_result);
    mocked.server.assert();

    Ok(())
}

#[tokio::test]
async fn get_game_high_scores_error() -> Result<(), Error> {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_game_high_scores_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = MockedAsync::new(&mut server, "getGameHighScores", 400, &mock_response);

    let mock_error = mocked.result_error()?;
    let params = GetGameHighScores {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked
        .client
        .get_game_high_scores(&params)
        .await
        .unwrap_err()
    {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }

    Ok(())
}
