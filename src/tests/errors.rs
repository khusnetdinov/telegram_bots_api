use crate::api::requests::sync::Requests;
use crate::api::responses::result::ResponseResult;
use crate::api::types::user::User;
use crate::clients::r#async::Async;
use crate::clients::sync::Sync;
use crate::config::Config;
use crate::errors::Error;
use crate::Client;

fn request_error() {
    todo!()
}

#[test]
fn response_error() {
    let config = Config {
        ..Default::default()
    };
    let client = Client {
        sync: Sync::new(&config),
        r#async: Async::new(&config),
        config,
    };
    let error = client.sync.get_me().unwrap_err();

    assert_eq!(
        error.to_string(),
        "Request Error: builder error: relative URL without a base"
    );
}

#[test]
fn decode_error() {
    let error = Error::Decode(serde_json::from_str::<ResponseResult<User>>("").unwrap_err());

    assert_eq!(
        error.to_string(),
        "Decode Error: eof while parsing a value at line 1 column 0"
    );
}

#[test]
fn unexpected_error() {
    let error = Error::Unexpected(String::from("with expected description"));

    assert_eq!(
        error.to_string(),
        "Unexpected Error: with expected description"
    );
}

#[test]
fn debug_error() {
    let error = Error::Debug;

    assert_eq!(error.to_string(), "Debug Error!");
}
