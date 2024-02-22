use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::file_input::FileInput;
use crate::api::enums::menu_button::MenuButton;
use crate::api::params::copy_message::CopyMessage;
use crate::api::params::copy_messages::CopyMessages;
use crate::api::params::delete_my_commands::DeleteMyCommands;
use crate::api::params::delete_webhook::DeleteWebhook;
use crate::api::params::forward_message::ForwardMessage;
use crate::api::params::forward_messages::ForwardMessages;
use crate::api::params::get_chat_menu_button::GetChatMenuButton;
use crate::api::params::get_forum_topic_icon_stickers::GetForumTopicIconStickers;
use crate::api::params::get_my_default_administrator_rights::GetMyDefaultAdministratorRights;
use crate::api::params::get_my_description::GetMyDescription;
use crate::api::params::get_my_short_description::GetMyShortDescription;
use crate::api::params::get_update::GetUpdate;
use crate::api::params::send_audio::SendAudio;
use crate::api::params::send_dice::SendDice;
use crate::api::params::send_message::SendMessage;
use crate::api::params::send_photo::SendPhoto;
use crate::api::params::set_chat_menu_button::SetChatMenuButton;
use crate::api::params::set_my_commands::SetMyCommands;
use crate::api::params::set_my_default_administrator_rights::SetMyDefaultAdministratorRights;
use crate::api::params::set_my_description::SetMyDescription;
use crate::api::params::set_my_short_description::SetMyShortDescription;
use crate::api::params::set_webhook::SetWebhook;
use crate::api::requests::sync::Requests;
use crate::api::responses::error::ResponseError;
use crate::api::types::bot_description::BotDescription;
use crate::api::types::bot_short_description::BotShortDescription;
use crate::api::types::chat_administrator_rights::ChatAdministratorRights;
use crate::api::types::message::Message;
use crate::api::types::message_id::MessageId;
use crate::api::types::sticker::Sticker;
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

// fn send_photo_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/send_photo_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendPhoto", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendPhoto {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_photo(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

#[test]
#[should_panic]
fn send_photo_error() {
    let mock_response = fs::read_to_string("src/tests/responses/send_photo_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "sendPhoto", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    let params = SendPhoto {
        chat_id: ChatUId::from(147951145),
        photo: FileInput::String(String::from("")),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.sync.send_photo(&params).unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

// fn send_audio_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/send_audio_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendAudio", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendAudio {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_audio(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

#[test]
#[should_panic]
fn send_audio_error() {
    let mock_response = fs::read_to_string("src/tests/responses/send_audio_error.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "sendAudio", &mock_response);

    let mock_error = mocked.result::<ResponseError>().unwrap();
    let params = SendAudio {
        chat_id: ChatUId::from(147951145),
        audio: FileInput::String(String::from("")),
        ..Default::default()
    };
    if let Error::Response(real_error) = mocked.client.sync.send_audio(&params).unwrap_err() {
        assert_eq!(mock_error, real_error);
        mocked.server.assert();
    }
}

// fn send_document_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/send_document_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendDocument", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendDocument {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_document(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_document_error() {
    todo!()
}

// fn send_video_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/send_video_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendVideo", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendVideo {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_video(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_video_error() {
    todo!()
}

// fn send_animation_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/send_animation_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendAnimation", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendAnimation {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_animation(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_animation_error() {
    todo!()
}

// fn send_voice_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/send_voice_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendVoice", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendVoice {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_voice(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_voice_error() {
    todo!()
}

// fn send_voice_note_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/send_video_note_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendVideoNote", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendVideoNote {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_video_note(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_voice_note_error() {
    todo!()
}

// fn send_media_group_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/send_media_group_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendMediaGroup", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendMediaGroup {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_media_group(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_media_group_error() {
    todo!()
}

// fn send_location_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/send_location_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendLocation", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendLocation {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_location(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_location_error() {
    todo!()
}

// fn send_venue_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/send_venue_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendVenue", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendVenue {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_venue(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_venue_error() {
    todo!()
}

// fn send_contact_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/send_contact_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendContact", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendContact {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_contact(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_contact_error() {
    todo!()
}

// fn send_poll_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/send_poll_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendPoll", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendPoll {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_poll(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

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

// fn send_chat_action_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/send_chat_action_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendChatAction", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SendChatAction {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_chat_action(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_chat_action_error() {
    todo!()
}

// fn set_message_reaction_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_message_reaction_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setMessageReaction", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetMessageReaction {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.set_message_reaction(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_message_reaction_error() {
    todo!()
}

// fn get_user_profile_photos_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/get_user_profile_photos_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "getUserProfilePhotos ", &mock_response);
//
//     let mock_result = mocked.result::<UserProfilePhotos>().unwrap();
//     let params = GetUserProfilePhotos {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.get_user_profile_photos(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn get_user_profile_photos_error() {
    todo!()
}

// fn get_file_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/get_file_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "getFile ", &mock_response);
//
//     let mock_result = mocked.result::<File>().unwrap();
//     let params = GetFile {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.get_file(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn get_file_error() {
    todo!()
}

// fn ban_chat_member_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/ban_chat_member_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "banChatMember", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = BanChatMember {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.ban_chat_member(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn ban_chat_member_error() {
    todo!()
}

// fn unban_chat_member_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/unban_chat_member_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "unbanChatMember", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = UnbanChatMember {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.unban_chat_member(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn unban_chat_member_error() {
    todo!()
}

// fn restrict_chat_member_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/restrict_chat_member_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "restrictChatMember", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = RestrictChatMember {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.restrict_chat_member(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn restrict_chat_member_error() {
    todo!()
}

// fn promote_chat_member_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/promote_chat_member_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "promoteChatMember", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = PromoteChatMember {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.promote_chat_member(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn promote_chat_member_error() {
    todo!()
}

// fn set_chat_administrator_custom_title_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_chat_administrator_custom_title_success.json")
//             .unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(
//         &mut server,
//         "setChatAdministratorCustomTitle",
//         &mock_response,
//     );
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetChatAdministratorCustomTitle {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .set_chat_administrator_custom_title(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_chat_administrator_custom_title_error() {
    todo!()
}

// fn ban_chat_sender_chat_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/ban_chat_sender_chat_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "banChatSenderChat", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = BanChatSenderChat {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.ban_chat_sender_chat(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn ban_chat_sender_chat_error() {
    todo!()
}

// fn unban_chat_sender_chat_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/unban_chat_sender_chat_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "unbanChatSenderChat", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = UnbanChatSenderChat {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.unban_chat_sender_chat(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn unban_chat_sender_chat_error() {
    todo!()
}

// fn set_chat_permissions_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_chat_permissions_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setChatPermissions", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetChatPermissions {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.set_chat_permissions(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_chat_permissions_error() {
    todo!()
}

// fn export_chat_invite_link_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/export_chat_invite_link_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "exportChatInviteLink", &mock_response);
//
//     let mock_result = mocked.result::<String>().unwrap();
//     let params = ExportChatInviteLink {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.export_chat_invite_link(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn export_chat_invite_link_error() {
    todo!()
}

// fn create_chat_invite_link_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/create_chat_invite_link_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "createChatInviteLink", &mock_response);
//
//     let mock_result = mocked.result::<ChatInviteLink>().unwrap();
//     let params = CreateChatInviteLink {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.create_chat_invite_link(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn create_chat_invite_link_error() {
    todo!()
}

// fn edit_chat_invite_link_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/edit_chat_invite_link_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "editChatInviteLink", &mock_response);
//
//     let mock_result = mocked.result::<ChatInviteLink>().unwrap();
//     let params = EditChatInviteLink {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.edit_chat_invite_link(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn edit_chat_invite_link_error() {
    todo!()
}

// fn revoke_chat_invite_link_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/revoke_chat_invite_link_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "revokeChatInviteLink", &mock_response);
//
//     let mock_result = mocked.result::<ChatInviteLink>().unwrap();
//     let params = RevokeChatInviteLink {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.revoke_chat_invite_link(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn revoke_chat_invite_link_error() {
    todo!()
}

// fn approve_chat_join_request_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/approve_chat_join_request_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "approveChatJoinRequest", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = ApproveChatJoinRequest {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .approve_chat_join_request(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn approve_chat_join_request_error() {
    todo!()
}

// fn decline_chat_join_request_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/decline_chat_join_request_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "declineChatJoinRequest", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = DeclineChatJoinRequest {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .decline_chat_join_request(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn decline_chat_join_request_error() {
    todo!()
}

// fn set_chat_photo_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_chat_photo_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setChatPhoto", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetChatPhoto {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.set_chat_photo(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_chat_photo_error() {
    todo!()
}

// fn delete_chat_photo_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/delete_chat_photo_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "deleteChatPhoto", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = DeleteChatPhoto {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.delete_chat_photo(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn delete_chat_photo_error() {
    todo!()
}

// fn set_chat_title_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_chat_title_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setChatTitle", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetChatTitle {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.set_chat_title(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_chat_title_error() {
    todo!()
}

// fn set_chat_description_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_chat_description_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setChatDescription", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetChatDescription {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.set_chat_description(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_chat_description_error() {
    todo!()
}

// fn pin_chat_message_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/pin_chat_message_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "pinChatMessage ", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = PinChatMessage {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.pin_chat_message(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn pin_chat_message_error() {
    todo!()
}

// fn unpin_chat_message_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/unpin_chat_message_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "unpinChatMessage", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = UnpinChatMessage {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.unpin_chat_message(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn unpin_chat_message_error() {
    todo!()
}

// fn unpin_all_chat_messages_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/unpin_all_chat_messages_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "unpinAllChatMessages", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = UnpinAllChatMessages {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.unpin_all_chat_messages(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn unpin_all_chat_messages_error() {
    todo!()
}

// fn leave_chat_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/leave_chat_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "leaveChat", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = LeaveChat {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.leave_chat(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn leave_chat_error() {
    todo!()
}

// fn get_chat_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/get_chat_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "getChat", &mock_response);
//
//     let mock_result = mocked.result::<Chat>().unwrap();
//     let params = GetChat {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.get_chat(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn get_chat_error() {
    todo!()
}

// fn get_chat_administrators_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/get_chat_administrators_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "getChatAdministrators", &mock_response);
//
//     let mock_result = mocked.result::<Vec<ChatMember>>().unwrap();
//     let params = GetChatAdministrators {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.get_chat_administrators(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn get_chat_administrators_error() {
    todo!()
}

// fn get_chat_member_count_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/get_chat_member_count_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "getChatMemberCount", &mock_response);
//
//     let mock_result = mocked.result::<i64>().unwrap();
//     let params = GetChatMemberCount {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.get_chat_member_count(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn get_chat_member_count_error() {
    todo!()
}

// fn get_chat_member_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/get_chat_member_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "getChatMember", &mock_response);
//
//     let mock_result = mocked.result::<ChatMember>().unwrap();
//     let params = GetChatMember {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.get_chat_member(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn get_chat_member_error() {
    todo!()
}

// fn set_chat_sticker_set_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_chat_sticker_set_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setChatStickerSet", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetChatStickerSet {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.set_chat_sticker_set(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_chat_sticker_set_error() {
    todo!()
}

// fn delete_chat_sticker_set_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/delete_chat_sticker_set_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "deleteChatStickerSet", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = DeleteChatStickerSet {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.delete_chat_sticker_set(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn delete_chat_sticker_set_error() {
    todo!()
}

#[test]
fn get_forum_topic_icon_stickers_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_forum_topic_icon_stickers.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "getForumTopicIconStickers", &mock_response);

    let mock_result = mocked.result::<Vec<Sticker>>().unwrap();
    let params = GetForumTopicIconStickers {
        ..Default::default()
    };
    let real_result = mocked
        .client
        .sync
        .get_forum_topic_icon_stickers(&params)
        .unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

// fn create_forum_topic_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/create_forum_topic_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "createForumTopic", &mock_response);
//
//     let mock_result = mocked.result::<ForumTopic>().unwrap();
//     let params = CreateForumTopic {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.create_forum_topic(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn create_forum_topic_error() {
    todo!()
}

// fn edit_forum_topic_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/edit_forum_topic_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "editForumTopic", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = EditForumTopic {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.edit_forum_topic(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn edit_forum_topic_error() {
    todo!()
}

// fn close_forum_topic_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/close_forum_topic_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "closeForumTopic", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = CloseForumTopic {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.close_forum_topic(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn close_forum_topic_error() {
    todo!()
}

// fn reopen_forum_topic_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/reopen_forum_topic_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "reopenForumTopic", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = ReopenForumTopic {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.reopen_forum_topic(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn reopen_forum_topic_error() {
    todo!()
}

// fn delete_forum_topic_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/delete_forum_topic_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "deleteForumTopic", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = DeleteForumTopic {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.delete_forum_topic(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn delete_forum_topic_error() {
    todo!()
}

// fn unpin_all_forum_topic_messages_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/unpin_all_forum_topic_messages_success.json")
//             .unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "unpinAllForumTopicMessages", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = UnpinAllForumTopicMessages {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .unpin_all_forum_topic_messages(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn unpin_all_forum_topic_messages_error() {
    todo!()
}

// fn edit_general_forum_topic_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/edit_general_forum_topic_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "editGeneralForumTopic", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = EditGeneralForumTopic {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .edit_general_forum_topic(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn edit_general_forum_topic_error() {
    todo!()
}

// fn close_general_forum_topic_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/close_general_forum_topic_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "closeGeneralForumTopic", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = CloseGeneralForumTopic {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .close_general_forum_topic(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn close_general_forum_topic_error() {
    todo!()
}

// fn reopen_general_forum_topic_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/reopen_general_forum_topic_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "reopenGeneralForumTopic", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = ReopenGeneralForumTopic {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .reopen_general_forum_topic(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn reopen_general_forum_topic_error() {
    todo!()
}

// fn hide_general_forum_topic_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/hide_general_forum_topic_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "hideGeneralForumTopic", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = HideGeneralForumTopic {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .hide_general_forum_topic(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn hide_general_forum_topic_error() {
    todo!()
}

// fn unhide_general_forum_topic_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/unhide_general_forum_topic_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "unhideGeneralForumTopic", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = UnhideGeneralForumTopic {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .unhide_general_forum_topic(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn unhide_general_forum_topic_error() {
    todo!()
}

// fn unpin_all_general_forum_topic_messages_success() {
//     let mock_response = fs::read_to_string(
//         "src/tests/responses/unpin_all_general_forum_topic_messages_success.json",
//     )
//     .unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(
//         &mut server,
//         "unpinAllGeneralForumTopicMessages",
//         &mock_response,
//     );
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = UnpinAllGeneralForumTopicMessages {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .unpin_all_general_forum_topic_messages(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn unpin_all_general_forum_topic_messages_error() {
    todo!()
}

// fn answer_callback_query_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/answer_callback_query_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "answerCallbackQuery", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = AnswerCallbackQuery {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.answer_callback_query(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn answer_callback_query_error() {
    todo!()
}

// fn get_user_chat_boosts_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/get_user_chat_boosts_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "getUserChatBoosts", &mock_response);
//
//     let mock_result = mocked.result::<UserChatBoosts>().unwrap();
//     let params = GetUserChatBoosts {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.get_user_chat_boosts(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn get_user_chat_boosts_error() {
    todo!()
}

#[test]
fn set_my_commands_success() {
    let mock_response = fs::read_to_string("src/tests/responses/set_my_commands.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "setMyCommands", &mock_response);

    let mock_result = mocked.result::<bool>().unwrap();
    let params = SetMyCommands {
        ..Default::default()
    };
    let real_result = mocked.client.sync.set_my_commands(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
fn delete_my_commands_success() {
    let mock_response = fs::read_to_string("src/tests/responses/delete_my_commands.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "deleteMyCommands", &mock_response);

    let mock_result = mocked.result::<bool>().unwrap();
    let params = DeleteMyCommands {
        ..Default::default()
    };
    let real_result = mocked.client.sync.delete_my_commands(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

// fn get_my_commands_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/get_my_commands.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "getMyCommands", &mock_response);
//
//     let mock_result = mocked.result::<Vec<BotCommand>>().unwrap();
//     let params = GetMyCommands {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.get_my_commands(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn get_my_commands_error() {
    todo!()
}

// fn set_my_name_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/set_my_name_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setMyName", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetMyName {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.set_my_name(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_my_name_error() {
    todo!()
}

// fn get_my_name_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/get_my_name_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "getMyName", &mock_response);
//
//     let mock_result = mocked.result::<BotName>().unwrap();
//     let params = GetMyName {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.get_my_name(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn get_my_name_error() {
    todo!()
}

#[test]
fn set_my_description_success() {
    let mock_response = fs::read_to_string("src/tests/responses/set_my_description.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "setMyDescription", &mock_response);

    let mock_result = mocked.result::<bool>().unwrap();
    let params = SetMyDescription {
        ..Default::default()
    };
    let real_result = mocked.client.sync.set_my_description(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
fn get_my_description_success() {
    let mock_response = fs::read_to_string("src/tests/responses/get_my_description.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "getMyDescription", &mock_response);

    let mock_result = mocked.result::<BotDescription>().unwrap();
    let params = GetMyDescription {
        ..Default::default()
    };
    let real_result = mocked.client.sync.get_my_description(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
fn set_my_short_description_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_my_short_description.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "setMyShortDescription", &mock_response);

    let mock_result = mocked.result::<bool>().unwrap();
    let params = SetMyShortDescription {
        ..Default::default()
    };
    let real_result = mocked
        .client
        .sync
        .set_my_short_description(&params)
        .unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
fn get_my_short_description_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_my_short_description.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "getMyShortDescription", &mock_response);

    let mock_result = mocked.result::<BotShortDescription>().unwrap();
    let params = GetMyShortDescription {
        ..Default::default()
    };
    let real_result = mocked
        .client
        .sync
        .get_my_short_description(&params)
        .unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
fn set_chat_menu_button_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_chat_menu_button.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "setChatMenuButton", &mock_response);

    let mock_result = mocked.result::<bool>().unwrap();
    let params = SetChatMenuButton {
        ..Default::default()
    };
    let real_result = mocked.client.sync.set_chat_menu_button(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
fn get_chat_menu_button_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_chat_menu_button.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(&mut server, "getChatMenuButton", &mock_response);

    let mock_result = mocked.result::<MenuButton>().unwrap();
    let params = GetChatMenuButton {
        ..Default::default()
    };
    let real_result = mocked.client.sync.get_chat_menu_button(&params).unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
fn set_my_default_administrator_rights_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/set_my_default_administrator_rights.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(
        &mut server,
        "setMyDefaultAdministratorRights",
        &mock_response,
    );

    let mock_result = mocked.result::<bool>().unwrap();
    let params = SetMyDefaultAdministratorRights {
        ..Default::default()
    };
    let real_result = mocked
        .client
        .sync
        .set_my_default_administrator_rights(&params)
        .unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

#[test]
fn get_my_default_administrator_rights_success() {
    let mock_response =
        fs::read_to_string("src/tests/responses/get_my_default_administrator_rights.json").unwrap();
    let mut server = mockito::Server::new();
    let mocked = Mocked::new(
        &mut server,
        "getMyDefaultAdministratorRights",
        &mock_response,
    );

    let mock_result = mocked.result::<ChatAdministratorRights>().unwrap();
    let params = GetMyDefaultAdministratorRights {
        ..Default::default()
    };
    let real_result = mocked
        .client
        .sync
        .get_my_default_administrator_rights(&params)
        .unwrap();

    assert_eq!(mock_result, real_result);
    mocked.server.assert();
}

// fn edit_message_text_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/edit_message_text_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "editMessageText", &mock_response);
//
//     let mock_result = mocked.result::<MessageResult>().unwrap();
//     let params = EditMessageText {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.edit_message_text(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn edit_message_text_error() {
    todo!()
}

// fn edit_message_caption_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/edit_message_caption_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "editMessageCaption", &mock_response);
//
//     let mock_result = mocked.result::<MessageResult>().unwrap();
//     let params = EditMessageCaption {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.edit_message_caption(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn edit_message_caption_error() {
    todo!()
}

// fn edit_message_media_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/edit_message_media_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "editMessageMedia", &mock_response);
//
//     let mock_result = mocked.result::<MessageResult>().unwrap();
//     let params = EditMessageMedia {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.edit_message_media(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn edit_message_media_error() {
    todo!()
}

// fn edit_message_live_location_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/edit_message_live_location_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "editMessageLiveLocation", &mock_response);
//
//     let mock_result = mocked.result::<MessageResult>().unwrap();
//     let params = EditMessageLiveLocation {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .edit_message_live_location(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn edit_message_live_location_error() {
    todo!()
}

// fn stop_message_live_location_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/stop_message_live_location_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "stopMessageLiveLocation", &mock_response);
//
//     let mock_result = mocked.result::<MessageResult>().unwrap();
//     let params = StopMessageLiveLocation {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .stop_message_live_location(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn stop_message_live_location_error() {
    todo!()
}

// fn edit_message_reply_markup_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/edit_message_reply_markup_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "editMessageReplyMarkup", &mock_response);
//
//     let mock_result = mocked.result::<MessageResult>().unwrap();
//     let params = EditMessageReplyMarkup {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .edit_message_reply_markup(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn edit_message_reply_markup_error() {
    todo!()
}

// fn stop_poll_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/stop_poll_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "stopPoll", &mock_response);
//
//     let mock_result = mocked.result::<Poll>().unwrap();
//     let params = StopPoll {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.stop_poll(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn stop_poll_error() {
    todo!()
}

// fn delete_message_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/delete_message_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "deleteMessage", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = DeleteMessage {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.delete_message(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn delete_message_error() {
    todo!()
}

// fn delete_messages_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/delete_messages_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "deleteMessages", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = DeleteMessages {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.delete_messages(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn delete_messages_error() {
    todo!()
}

// fn send_sticker_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/send_sticker_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendSticker", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendSticker {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_sticker(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_sticker_error() {
    todo!()
}

// fn get_sticker_set_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/get_sticker_set_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "getStickerSet", &mock_response);
//
//     let mock_result = mocked.result::<StickerSet>().unwrap();
//     let params = GetStickerSet {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.get_sticker_set(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn get_sticker_set_error() {
    todo!()
}

// fn get_custom_emoji_stickers_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/get_custom_emoji_stickers_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "getCustomEmojiStickers", &mock_response);
//
//     let mock_result = mocked.result::<Sticker>().unwrap();
//     let params = GetCustomEmojiStickers {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .get_custom_emoji_stickers(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn get_custom_emoji_stickers_error() {
    todo!()
}

// fn upload_sticker_file_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/upload_sticker_file_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "uploadStickerFile", &mock_response);
//
//     let mock_result = mocked.result::<File>().unwrap();
//     let params = UploadStickerFile {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.upload_sticker_file(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn upload_sticker_file_error() {
    todo!()
}

// fn create_new_sticker_set_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/create_new_sticker_set_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "createNewStickerSet", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = CreateNewStickerSet {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.create_new_sticker_set(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn create_new_sticker_set_error() {
    todo!()
}

// fn add_sticker_to_set_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/add_sticker_to_set_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "addStickerToSet", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = AddStickerToSet {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.add_sticker_to_set(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn add_sticker_to_set_error() {
    todo!()
}

// fn set_sticker_position_in_set_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_sticker_position_in_set_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setStickerPositionInSet", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetStickerPositionInSet {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .set_sticker_position_in_set(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_sticker_position_in_set_error() {
    todo!()
}

// fn delete_sticker_from_set_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/delete_sticker_from_set_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "deleteStickerFromSet", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = DeleteStickerFromSet {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.delete_sticker_from_set(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn delete_sticker_from_set_error() {
    todo!()
}

// fn set_sticker_emoji_list_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_sticker_emoji_list_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setStickerEmojiList", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetStickerEmojiList {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.set_sticker_emoji_list(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_sticker_emoji_list_error() {
    todo!()
}

// fn set_sticker_keywords_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_sticker_keywords_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setStickerKeywords", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetStickerKeywords {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.set_sticker_keywords(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_sticker_keywords_error() {
    todo!()
}

// fn set_sticker_mask_position_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_sticker_mask_position_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setStickerMaskPosition", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetStickerMaskPosition {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .set_sticker_mask_position(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_sticker_mask_position_error() {
    todo!()
}

// fn set_sticker_set_title_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_sticker_set_title_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setStickerSetTitle", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetStickerSetTitle {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.set_sticker_set_title(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_sticker_set_title_error() {
    todo!()
}

// fn set_sticker_set_thumbnail_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_sticker_set_thumbnail_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setStickerSetThumbnail", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetStickerSetThumbnail {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .set_sticker_set_thumbnail(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_sticker_set_thumbnail_error() {
    todo!()
}

// fn set_custom_emoji_sticker_set_thumbnail_success() {
//     let mock_response = fs::read_to_string(
//         "src/tests/responses/set_custom_emoji_sticker_set_thumbnail_success.json",
//     )
//     .unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(
//         &mut server,
//         "setCustomEmojiStickerSetThumbnail",
//         &mock_response,
//     );
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetCustomEmojiStickerSetThumbnail {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .set_custom_emoji_sticker_set_thumbnail(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_custom_emoji_sticker_set_thumbnail_error() {
    todo!()
}

// fn delete_sticker_set_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/delete_sticker_set_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "deleteStickerSet", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = DeleteStickerSet {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.delete_sticker_set(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn delete_sticker_set_error() {
    todo!()
}

// fn answer_inline_query_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/answer_inline_query_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "answerInlineQuery", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = AnswerInlineQuery {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.answer_inline_query(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn answer_inline_query_error() {
    todo!()
}

// fn answer_web_app_query_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/answer_web_app_query_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "answerWebAppQuery", &mock_response);
//
//     let mock_result = mocked.result::<SentWebAppMessage>().unwrap();
//     let params = AnswerWebAppQuery {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.answer_web_app_query(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn answer_web_app_query_error() {
    todo!()
}

// fn send_invoice_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/send_invoice_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendInvoice", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendInvoice {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_invoice(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_invoice_error() {
    todo!()
}

// fn create_invoice_link_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/create_invoice_link_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "createInvoiceLink", &mock_response);
//
//     let mock_result = mocked.result::<String>().unwrap();
//     let params = CreateInvoiceLink {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.create_invoice_link(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn create_invoice_link_error() {
    todo!()
}

// fn answer_shipping_query_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/answer_shipping_query_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "answerShippingQuery", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = AnswerShippingQuery {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.answer_shipping_query(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn answer_shipping_query_error() {
    todo!()
}

// fn answer_pre_checkout_query_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/answer_pre_checkout_query_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "answerPreCheckoutQuery", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = AnswerPreCheckoutQuery {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .answer_pre_checkout_query(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn answer_pre_checkout_query_error() {
    todo!()
}

// fn set_passport_data_errors_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_passport_data_errors_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setPassportDataErrors", &mock_response);
//
//     let mock_result = mocked.result::<bool>().unwrap();
//     let params = SetPassportDataErrors {
//         ..Default::default()
//     };
//     let real_result = mocked
//         .client
//         .sync
//         .set_passport_data_errors(&params)
//         .unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_passport_data_errors_error() {
    todo!()
}

// fn send_game_success() {
//     let mock_response = fs::read_to_string("src/tests/responses/send_game_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "sendGame", &mock_response);
//
//     let mock_result = mocked.result::<Message>().unwrap();
//     let params = SendGame {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.send_game(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn send_game_error() {
    todo!()
}

// fn set_game_score_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/set_game_score_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setGameScore", &mock_response);
//
//     let mock_result = mocked.result::<MessageResult>().unwrap();
//     let params = SetGameScore {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.set_game_score(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn set_game_score_error() {
    todo!()
}

// fn get_game_high_scores_success() {
//     let mock_response =
//         fs::read_to_string("src/tests/responses/get_game_high_scores_success.json").unwrap();
//     let mut server = mockito::Server::new();
//     let mocked = Mocked::new(&mut server, "setGameScore", &mock_response);
//
//     let mock_result = mocked.result::<GameHighScore>().unwrap();
//     let params = GetGameHighScores {
//         ..Default::default()
//     };
//     let real_result = mocked.client.sync.get_game_high_scores(&params).unwrap();
//
//     assert_eq!(mock_result, real_result);
//     mocked.server.assert();
// }

fn get_game_high_scores_error() {
    todo!()
}
