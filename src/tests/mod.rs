mod client;
mod clients;
mod config;
mod errors;

mod helpers {
    use crate::clients::r#async::Async;
    use crate::clients::sync::Sync;
    use crate::config::Config;
    use crate::Client;
    use mockito::{Mock, ServerGuard};

    pub fn mocked_token(token: Option<&str>) -> &str {
        token.unwrap_or("0000000000:XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")
    }

    pub fn mock_api(server: &ServerGuard, token: &str) -> Client {
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

    pub fn mock_server(
        server: &mut ServerGuard,
        token: &str,
        method: &str,
        response: &str,
    ) -> Mock {
        server
            .mock("POST", format!("/bot{}/{}", token, method).as_str())
            .match_header("content-type", "application/json")
            .with_body(response)
            .create()
    }
}
