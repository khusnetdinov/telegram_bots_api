use crate::api::responses::error::ResponseError;
use crate::api::responses::result::ResponseResult;
use crate::clients::sync::Sync;
use crate::config::Config;
use crate::errors::Error;
use mockito::{Mock, ServerGuard};
use serde::de::DeserializeOwned;

pub struct MockedSync {
    pub client: Sync,
    pub server: Mock,
    pub response: String,
}

impl MockedSync {
    fn mock_api(server: &ServerGuard, token: &str) -> Sync {
        let config = Config {
            url: server.url(),
            token: token.to_string(),
            ..Default::default()
        };

        Sync::from(&config)
    }

    fn mock_server(
        server: &mut ServerGuard,
        token: &str,
        method: &str,
        status: usize,
        response: &str,
    ) -> Mock {
        server
            .mock("POST", format!("/bot{}/{}", token, method).as_str())
            .match_header("content-type", "application/json")
            .with_body(response)
            .with_status(status)
            .create()
    }

    pub fn new(server: &mut ServerGuard, method: &str, status: usize, response: &str) -> Self {
        let token = "0000000000:XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
        let mocked_client = Self::mock_api(server, token);
        let mocked_server = Self::mock_server(server, token, method, status, response);

        Self {
            client: mocked_client,
            server: mocked_server,
            response: String::from(response),
        }
    }

    pub fn result<T: DeserializeOwned>(&self) -> Result<T, Error> {
        match serde_json::from_str::<ResponseResult<T>>(&self.response) {
            Ok(success) => Ok(success.result),
            Err(error) => Err(Error::Decode(error)),
        }
    }

    pub fn result_error(&self) -> Result<ResponseError, Error> {
        match serde_json::from_str::<ResponseError>(&self.response) {
            Ok(error) => Ok(error),
            Err(error) => Err(Error::Decode(error)),
        }
    }
}
