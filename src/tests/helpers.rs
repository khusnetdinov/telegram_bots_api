use crate::api::responses::error::ResponseError;
use crate::api::responses::result::ResponseResult;
use crate::clients::r#async::Async;
use crate::clients::sync::Sync;
use crate::config::Config;
use crate::errors::Error;
use crate::Client;
use mockito::{Mock, ServerGuard};
use serde::de::DeserializeOwned;

pub struct Mocked {
    pub client: Client,
    pub server: Mock,
    pub response: String,
}

impl Mocked {
    fn mock_api(server: &ServerGuard, token: &str) -> Client {
        let config = Config {
            url: server.url(),
            token: token.to_string(),
            ..Default::default()
        };

        Client {
            sync: Sync::new(&config),
            r#async: Async::new(&config),
            config,
        }
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
