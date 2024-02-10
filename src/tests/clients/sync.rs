
use crate::api::enums::chat_uid::ChatUId;
use crate::api::params::copy_message::CopyMessage;
use crate::api::params::copy_messages::CopyMessages;
use crate::api::params::delete_webhook::DeleteWebhook;
use crate::api::params::forward_message::ForwardMessage;
use crate::api::params::forward_messages::ForwardMessages;
use crate::api::params::get_update::GetUpdate;
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
