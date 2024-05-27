use crate::api::requests::sync::Requests;
use crate::api::responses::error::ResponseError;
use crate::api::responses::result::ResponseResult;
use crate::api::structs::user::User;
use crate::clients::sync::Sync;
use crate::config::Config;
use crate::errors::Error;

#[test]
fn request_error() {
    let error = Error::Response(ResponseError {
        ok: false,
        error_code: 000,
        description: String::from("description"),
        parameters: None,
    });

    assert_eq!(
        error.to_string(),
        "Response Error: error code 0: description"
    );
}

#[test]
fn response_sync_error() {
    let config = Config {
        ..Default::default()
    };
    let client = Sync::from(&config);
    let error = client.get_me().unwrap_err();

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
