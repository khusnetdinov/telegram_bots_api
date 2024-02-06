#[cfg(test)]
mod tests {
    use crate::api::requests::sync::Requests;
    use crate::api::responses::result::ResponseResult;
    use crate::api::types::user::User;
    use crate::clients::r#async::Async;
    use crate::clients::sync::Sync;
    use crate::config::Config;
    use crate::Client;

    // #[test]
    // fn example() {
    //     let mock_method = "getMe";
    //     let mock_response =  "{\"ok\":true,\"result\":{\"id\":6591790550,\"is_bot\":true,\"first_name\":\"development\",\"username\":\"development_bot\",\"can_join_groups\":true,\"can_read_all_group_messages\":false,\"supports_inline_queries\":false}}";
    //
    //     let mut server = mockito::Server::new();
    //     let mock_server_url = server.url();
    //     let mock_token = "0000000000:XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    //     let mock_url = format!("/bot{}/{}", mock_token, mock_method);
    //     // let real_server_url = "https://api.telegram.org";
    //     // let real_token = "6591790550:AAE5s6Mmhs8QsGPDxmTxEB23kvKKg3KrI_w";
    //
    //     let config = Config {
    //         url: String::from(mock_server_url),
    //         token: String::from(mock_token),
    //         ..Default::default()
    //     };
    //
    //     let api = Client {
    //         sync: Sync::new(&config),
    //         r#async: Async::new(&config),
    //         config,
    //     };
    //
    //     let mock = server
    //         .mock("POST", mock_url.as_str())
    //         .match_header("content-type", "application/json")
    //         .with_body(mock_response)
    //         .create();
    //
    //     // mock:begin
    //     // let real_url = format!("/bot{}/{}", real_token, mock_method);
    //     // let timeout = 5u64;
    //     // let connect_timeout = 5u64;
    //     // let client = ClientBuilder::new()
    //     //     .timeout(Duration::from_secs(timeout))
    //     //     .connect_timeout(Duration::from_secs(connect_timeout))
    //     //     .build()
    //     //     .unwrap();
    //
    //     // let mock_path = format!("{}/bot{}/{}", mock_server_url, mock_token, "getMe");
    //     // let real_path = format!("{}/bot{}/{}", real_server_url, real_token, "getMe");
    //     // let request = client.post(&mock_path).json(&{});
    //     // let response = request.send().unwrap().text().unwrap();
    //     // mock:end
    //
    //     let mock_user = serde_json::from_str::<ResponseResult<User>>(&mock_response)
    //         .unwrap()
    //         .result;
    //     let real_user = api.sync.get_me().unwrap();
    //     // let real_user = serde_json::from_str::<ResponseResult<User>>(&response).unwrap().result;
    //
    //     assert_eq!(mock_user, real_user);
    //     mock.assert();
    // }

    #[test]
    fn get_me() {
        let mock_method = "getMe";
        let mock_response =  "{\"ok\":true,\"result\":{\"id\":6591790550,\"is_bot\":true,\"first_name\":\"development\",\"username\":\"development_bot\",\"can_join_groups\":true,\"can_read_all_group_messages\":false,\"supports_inline_queries\":false}}";

        let mut server = mockito::Server::new();
        let mock_server_url = server.url();
        let mock_token = String::from("0000000000:XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        let mock_url = format!("/bot{}/{}", mock_token, mock_method);

        let config = Config {
            url: mock_server_url,
            token: mock_token,
            ..Default::default()
        };

        let api = Client {
            sync: Sync::new(&config),
            r#async: Async::new(&config),
            config,
        };

        let mock = server
            .mock("POST", mock_url.as_str())
            .match_header("content-type", "application/json")
            .with_body(mock_response)
            .create();

        let mock_user = serde_json::from_str::<ResponseResult<User>>(mock_response)
            .unwrap()
            .result;
        let real_user = api.sync.get_me().unwrap();

        assert_eq!(mock_user, real_user);
        mock.assert();
    }
}
