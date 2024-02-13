use crate::api::enums::chat_uid::ChatUId;
use crate::api::params::copy_message::CopyMessage;
use crate::api::params::copy_messages::CopyMessages;
use crate::api::params::delete_webhook::DeleteWebhook;
use crate::api::params::forward_message::ForwardMessage;
use crate::api::params::forward_messages::ForwardMessages;
use crate::api::params::get_update::GetUpdate;
use crate::api::params::send_dice::SendDice;
use crate::api::params::send_message::SendMessage;
use crate::api::params::set_webhook::SetWebhook;
use crate::api::requests::sync::Requests;
use crate::api::responses::error::ResponseError;
use crate::api::types::message::Message;
use crate::api::types::message_id::MessageId;
use crate::api::types::update::Update;
use crate::api::types::user::User;
use crate::api::types::webhook_info::WebhookInfo;
use crate::errors::Error;
use crate::tests::helpers::*;
use std::fs;

#[test]
fn get_updates_success() {
    let mock_response = fs::read_to_string("src/tests/responses/get_updates_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "getUpdates", &mock_response);

    let mock_result = mocked.result::<Vec<Update>>().unwrap();
    let params = GetUpdate {
        limit: 100,
        offset: 249563340,
        timeout: 0,
        ..Default::default()
    };
    let real_result = mocked.client.sync.get_updates(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn get_updates_error() {
    let mock_response = fs::read_to_string("src/tests/responses/get_updates_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "getUpdates", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    let params = GetUpdate {
        limit: 100,
        offset: 249563340,
        timeout: 0,
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.sync.get_updates(&params).unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

#[test]
fn set_webhook_success() {
    let mock_response = fs::read_to_string("src/tests/responses/set_webhook_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "setWebhook", &mock_response);

    let mock_result = mocked.result::<bool>().unwrap();
    let params = SetWebhook {
        url: String::from("https://78b3-91-202-26-13.ngrok-free.app"),
        ..Default::default()
    };
    let real_result = mocked.client.sync.set_webhook(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn set_webhook_error() {
    let mock_response = fs::read_to_string("src/tests/responses/set_webhook_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "setWebhook", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    let params = SetWebhook {
        url: String::from("https://78b3-91-202-26-13.ngrok-free.app"),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.sync.set_webhook(&params).unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

#[test]
fn delete_webhook_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_webhook_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "deleteWebhook", &mock_response);

    let mock_result = mocked.result::<bool>().unwrap();
    let params = DeleteWebhook {
        ..Default::default()
    };
    let real_result = mocked.client.sync.delete_webhook(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn delete_webhook_error() {
    let mock_response =
        fs::read_to_string("src/tests/responses/delete_webhook_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "deleteWebhook", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    let params = DeleteWebhook {
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.sync.delete_webhook(&params).unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

#[test]
fn get_webhook_info_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_webhook_info_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "getWebhookInfo", &mock_response);

    let mock_result = mocked.result::<WebhookInfo>().unwrap();
    let real_result = mocked.client.sync.get_webhook_info().unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn get_webhook_info_error() {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_webhook_info_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "deleteWebhook", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    if let Error::Response(real_error) = mocked.client.sync.get_webhook_info().unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

#[test]
fn get_me_success() {
    let mock_response = fs::read_to_string("src/tests/responses/get_me_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "getMe", &mock_response);

    let mock_result = mocked.result::<User>().unwrap();
    let real_result = mocked.client.sync.get_me().unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn get_me_error() {
    let mock_response = fs::read_to_string("src/tests/responses/get_me_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "getMe", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    if let Error::Response(real_error) = mocked.client.sync.get_me().unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

#[test]
fn log_out_success() {
    let mock_response = fs::read_to_string("src/tests/responses/log_out_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "logOut", &mock_response);

    let mock_result = mocked.result::<bool>().unwrap();
    let real_result = mocked.client.sync.log_out().unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn log_out_error() {
    let mock_response = fs::read_to_string("src/tests/responses/log_out_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "logOut", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    if let Error::Response(real_error) = mocked.client.sync.log_out().unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

#[test]
fn close_success() {
    let mock_response = fs::read_to_string("src/tests/responses/close_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "close", &mock_response);

    let mock_result = mocked.result::<bool>().unwrap();
    let real_result = mocked.client.sync.close().unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn close_error() {
    let mock_response = fs::read_to_string("src/tests/responses/close_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "close", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    if let Error::Response(real_error) = mocked.client.sync.close().unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

#[test]
fn send_message_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/send_message_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "sendMessage", &mock_response);

    let mock_result = mocked.result::<Message>().unwrap();
    let params = SendMessage {
        chat_id: ChatUId::from(147951145),
        text: "Hello World".to_string(),
        ..Default::default()
    };
    let real_result = mocked.client.sync.send_message(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn send_message_error() {
    let mock_response = fs::read_to_string("src/tests/responses/send_message_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "sendMessage", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    let params = SendMessage {
        chat_id: ChatUId::from(147951145),
        text: "Hello World".to_string(),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.sync.send_message(&params).unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

#[test]
fn forward_message_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/forward_message_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "forwardMessage", &mock_response);

    let mock_result = mocked.result::<MessageId>().unwrap();
    let params = ForwardMessage {
        message_id: MessageId::from(456),
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    let real_result = mocked.client.sync.forward_message(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn forward_message_error() {
    let mock_response =
        fs::read_to_string("src/tests/responses/forward_message_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "forwardMessage", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    let params = ForwardMessage {
        message_id: MessageId::from(456),
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.sync.forward_message(&params).unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

#[test]
fn forward_messages_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/forward_messages_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "forwardMessages", &mock_response);

    let mock_result = mocked.result::<Vec<MessageId>>().unwrap();
    let params = ForwardMessages {
        message_ids: vec![MessageId::from(456)],
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    let real_result = mocked.client.sync.forward_messages(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn forward_messages_error() {
    let mock_response =
        fs::read_to_string("src/tests/responses/forward_messages_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "forwardMessages", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    let params = ForwardMessages {
        message_ids: vec![MessageId::from(455), MessageId::from(456)],
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.sync.forward_messages(&params).unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

#[test]
fn copy_message_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/copy_message_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "copyMessage", &mock_response);

    let mock_result = mocked.result::<MessageId>().unwrap();
    let params = CopyMessage {
        message_id: MessageId::from(456),
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    let real_result = mocked.client.sync.copy_message(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn copy_message_error() {
    let mock_response = fs::read_to_string("src/tests/responses/copy_message_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "copyMessage", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    let params = CopyMessage {
        message_id: MessageId::from(456),
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.sync.copy_message(&params).unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

#[test]
fn copy_messages_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/copy_messages_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "copyMessages", &mock_response);

    let mock_result = mocked.result::<Vec<MessageId>>().unwrap();
    let params = CopyMessages {
        message_ids: vec![MessageId::from(456)],
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    let real_result = mocked.client.sync.copy_messages(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn copy_messages_error() {
    let mock_response = fs::read_to_string("src/tests/responses/copy_messages_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "copyMessages", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    let params = CopyMessages {
        message_ids: vec![MessageId::from(455), MessageId::from(456)],
        chat_id: ChatUId::from(147951145),
        from_chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.sync.copy_messages(&params).unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

fn send_photo_success() {
    todo!()
}

fn send_photo_error() {
    todo!()
}

fn send_audio_success() {
    todo!()
}

fn send_audio_error() {
    todo!()
}

fn send_document_success() {
    todo!()
}

fn send_document_error() {
    todo!()
}

fn send_video_success() {
    todo!()
}

fn send_video_error() {
    todo!()
}

fn send_animation_success() {
    todo!()
}

fn send_animation_error() {
    todo!()
}

fn send_voice_success() {
    todo!()
}

fn send_voice_error() {
    todo!()
}

fn send_voice_note_success() {
    todo!()
}

fn send_voice_note_error() {
    todo!()
}

fn send_media_group_success() {
    todo!()
}

fn send_media_group_error() {
    todo!()
}

fn send_location_success() {
    todo!()
}

fn send_location_error() {
    todo!()
}

fn send_venue_success() {
    todo!()
}

fn send_venue_error() {
    todo!()
}

fn send_contact_success() {
    todo!()
}

fn send_contact_error() {
    todo!()
}

fn send_poll_success() {
    todo!()
}

fn send_poll_error() {
    todo!()
}

#[test]
fn send_dice_success() {
    let mock_response = fs::read_to_string("src/tests/responses/send_dice_success.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "sendDice", &mock_response);

    let mock_result = mocked.result::<Message>().unwrap();
    let params = SendDice {
        chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    let real_result = mocked.client.sync.send_dice(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
#[should_panic]
fn send_dice_error() {
    let mock_response = fs::read_to_string("src/tests/responses/send_dice_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "sendDice", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    let params = SendDice {
        chat_id: ChatUId::from(147951145),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.sync.send_dice(&params).unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

fn send_chat_action_success() {
    todo!()
}

fn send_chat_action_error() {
    todo!()
}

fn get_user_profile_photos_success() {
    todo!()
}

fn get_user_profile_photos_error() {
    todo!()
}

fn get_file_success() {
    todo!()
}

fn get_file_error() {
    todo!()
}

fn ban_chat_member_success() {
    todo!()
}

fn ban_chat_member_error() {
    todo!()
}

fn unban_chat_member_success() {
    todo!()
}

fn unban_chat_member_error() {
    todo!()
}

fn restrict_chat_member_success() {
    todo!()
}

fn restrict_chat_member_error() {
    todo!()
}

fn promote_chat_member_success() {
    todo!()
}

fn promote_chat_member_error() {
    todo!()
}

fn send_chat_administrator_custom_title_success() {
    todo!()
}

fn send_chat_administrator_custom_title_error() {
    todo!()
}

fn ban_chat_sender_chat_success() {
    todo!()
}

fn ban_chat_sender_chat_error() {
    todo!()
}

fn unban_chat_sender_chat_success() {
    todo!()
}

fn unban_chat_sender_chat_error() {
    todo!()
}

fn set_chat_permissions_success() {
    todo!()
}

fn set_chat_permissions_error() {
    todo!()
}

fn export_chat_invite_link_success() {
    todo!()
}

fn export_chat_invite_link_error() {
    todo!()
}

fn create_chat_invite_link_success() {
    todo!()
}

fn create_chat_invite_link_error() {
    todo!()
}

fn edit_chat_invite_link_success() {
    todo!()
}

fn edit_chat_invite_link_error() {
    todo!()
}

fn revoke_chat_invite_link_success() {
    todo!()
}

fn remove_chat_invite_link_error() {
    todo!()
}

fn approve_chat_join_request_success() {
    todo!()
}

fn approve_chat_join_request_error() {
    todo!()
}

fn decline_chat_join_request_success() {
    todo!()
}

fn decline_chat_join_request_error() {
    todo!()
}

fn set_chat_photo_success() {
    todo!()
}

fn set_chat_photo_error() {
    todo!()
}

fn delete_chat_photo_success() {
    todo!()
}

fn delete_chat_photo_error() {
    todo!()
}

fn set_chat_title_success() {
    todo!()
}

fn set_chat_title_error() {
    todo!()
}

fn set_chat_description_success() {
    todo!()
}

fn set_chat_description_error() {
    todo!()
}

fn pin_chat_message_success() {
    todo!()
}

fn pin_chat_message_error() {
    todo!()
}

fn unpin_chat_message_success() {
    todo!()
}

fn unpin_chat_message_error() {
    todo!()
}

fn unpin_all_chat_messages_success() {
    todo!()
}

fn unpin_all_chat_messages_error() {
    todo!()
}

fn leave_chat_success() {
    todo!()
}

fn leave_chat_error() {
    todo!()
}

fn get_chat_success() {
    todo!()
}

fn get_chat_error() {
    todo!()
}

fn get_chat_administrators_success() {
    todo!()
}

fn get_chat_administrators_error() {
    todo!()
}

fn get_chat_member_count_success() {
    todo!()
}

fn get_chat_member_count_error() {
    todo!()
}

fn get_chat_member_success() {
    todo!()
}

fn get_chat_member_error() {
    todo!()
}

fn set_chat_sticker_set_success() {
    todo!()
}

fn set_chat_sticker_set_error() {
    todo!()
}

fn delete_chat_sticker_set_success() {
    todo!()
}

fn delete_chat_sticker_set_error() {
    todo!()
}

fn get_forum_topic_icon_stickers_success() {
    todo!()
}

fn get_forum_topic_icon_stickers_error() {
    todo!()
}

fn create_forum_topic_success() {
    todo!()
}

fn create_forum_topic_error() {
    todo!()
}

fn edit_forum_topic_success() {
    todo!()
}

fn edit_forum_topic_error() {
    todo!()
}

fn close_forum_topic_success() {
    todo!()
}

fn close_forum_topic_error() {
    todo!()
}

fn reopen_forum_topic_success() {
    todo!()
}

fn reopen_forum_topic_error() {
    todo!()
}

fn delete_forum_topic_success() {
    todo!()
}

fn delete_forum_topic_error() {
    todo!()
}

fn unpin_all_forum_topic_messages_success() {
    todo!()
}

fn unpin_all_forum_topic_messages_error() {
    todo!()
}

fn edit_general_forum_topic_success() {
    todo!()
}

fn edit_general_forum_topic_error() {
    todo!()
}

fn close_general_forum_topic_success() {
    todo!()
}

fn close_general_forum_topic_error() {
    todo!()
}

fn reopen_general_forum_topic_success() {
    todo!()
}

fn reopen_general_forum_topic_error() {
    todo!()
}

fn hide_general_forum_topic_success() {
    todo!()
}

fn hide_general_forum_topic_error() {
    todo!()
}

fn unhide_general_forum_topic_success() {
    todo!()
}

fn unhide_general_forum_topic_error() {
    todo!()
}

fn unpin_all_general_forum_topic_messages_success() {
    todo!()
}

fn unpin_all_general_forum_topic_messages_error() {
    todo!()
}

fn answer_callback_query_success() {
    todo!()
}

fn answer_callback_query_error() {
    todo!()
}

fn set_my_commands_success() {
    todo!()
}

fn set_my_commands_error() {
    todo!()
}

fn delete_my_commands_success() {
    todo!()
}

fn delete_my_commands_error() {
    todo!()
}

fn get_my_commands_success() {
    todo!()
}

fn get_my_commands_error() {
    todo!()
}

fn set_my_name_success() {
    todo!()
}

fn set_my_name_error() {
    todo!()
}

fn get_my_name_success() {
    todo!()
}

fn get_my_name_error() {
    todo!()
}

fn set_my_description_success() {
    todo!()
}

fn set_my_description_error() {
    todo!()
}

fn get_my_description_success() {
    todo!()
}

fn get_my_description_error() {
    todo!()
}

fn set_my_short_description_success() {
    todo!()
}

fn set_my_short_description_error() {
    todo!()
}

fn get_my_short_description_success() {
    todo!()
}

fn get_my_short_description_error() {
    todo!()
}

fn set_chat_menu_button_success() {
    todo!()
}

fn set_chat_menu_button_error() {
    todo!()
}

fn get_chat_menu_button_success() {
    todo!()
}

fn get_chat_menu_button_error() {
    todo!()
}

fn set_my_default_administrator_rights_success() {
    todo!()
}

fn set_my_default_administrator_rights_error() {
    todo!()
}

fn get_my_default_administrator_rights_success() {
    todo!()
}

fn get_my_default_administrator_rights_error() {
    todo!()
}

fn edit_message_text_success() {
    todo!()
}

fn edit_message_text_error() {
    todo!()
}

fn edit_message_caption_success() {
    todo!()
}

fn edit_message_caption_error() {
    todo!()
}

fn edit_message_media_success() {
    todo!()
}

fn edit_message_media_error() {
    todo!()
}

fn edit_message_live_location_success() {
    todo!()
}

fn edit_message_live_location_error() {
    todo!()
}

fn stop_message_live_location_success() {
    todo!()
}

fn stop_message_live_location_error() {
    todo!()
}

fn edit_message_reply_markup_success() {
    todo!()
}

fn edit_message_reply_markup_error() {
    todo!()
}

fn stop_poll_success() {
    todo!()
}

fn stop_poll_error() {
    todo!()
}

fn delete_message_success() {
    todo!()
}

fn delete_message_error() {
    todo!()
}

fn send_sticker_success() {
    todo!()
}

fn send_sticker_error() {
    todo!()
}

fn get_sticker_set_success() {
    todo!()
}

fn get_sticker_set_error() {
    todo!()
}

fn get_custom_emoji_stickers_success() {
    todo!()
}

fn get_custom_emoji_stickers_error() {
    todo!()
}

fn upload_sticker_file_success() {
    todo!()
}

fn upload_sticker_file_error() {
    todo!()
}

fn create_new_sticker_set_success() {
    todo!()
}

fn create_new_sticker_set_error() {
    todo!()
}

fn add_sticker_to_set_success() {
    todo!()
}

fn add_sticker_to_set_error() {
    todo!()
}

fn set_sticker_position_in_set_success() {
    todo!()
}

fn set_sticker_position_in_set_error() {
    todo!()
}

fn delete_sticker_from_set_success() {
    todo!()
}

fn delete_sticker_from_set_error() {
    todo!()
}

fn set_sticker_emoji_list_success() {
    todo!()
}

fn set_sticker_emoji_list_error() {
    todo!()
}

fn set_sticker_keywords_success() {
    todo!()
}

fn set_sticker_keywords_error() {
    todo!()
}

fn set_sticker_mask_position_success() {
    todo!()
}

fn set_sticker_mask_position_error() {
    todo!()
}

fn set_sticker_set_title_success() {
    todo!()
}

fn set_sticker_set_title_error() {
    todo!()
}

fn set_sticker_set_thumbnail_success() {
    todo!()
}

fn set_sticker_set_thumbnail_error() {
    todo!()
}

fn set_custom_emoji_sticker_set_thumbnail_success() {
    todo!()
}

fn set_custom_emoji_sticker_set_thumbnail_error() {
    todo!()
}

fn delete_sticker_set_success() {
    todo!()
}

fn delete_sticker_set_error() {
    todo!()
}

fn answer_inline_query_success() {
    todo!()
}

fn answer_inline_query_error() {
    todo!()
}

fn answer_web_app_query_success() {
    todo!()
}

fn answer_web_app_query_error() {
    todo!()
}

fn send_invoice_success() {
    todo!()
}

fn send_invoice_error() {
    todo!()
}

fn create_invoice_link_success() {
    todo!()
}

fn create_invoice_link_error() {
    todo!()
}

fn answer_shipping_query_success() {
    todo!()
}

fn answer_shipping_query_error() {
    todo!()
}

fn answer_pre_checkout_query_success() {
    todo!()
}

fn answer_pre_checkout_query_error() {
    todo!()
}

fn set_passport_data_errors_success() {
    todo!()
}

fn set_passport_data_errors_error() {
    todo!()
}

fn send_game_success() {
    todo!()
}

fn send_game_error() {
    todo!()
}

fn set_game_score_success() {
    todo!()
}

fn set_game_score_error() {
    todo!()
}

fn get_game_high_scores_success() {
    todo!()
}

fn get_game_high_scores_error() {
    todo!()
}
