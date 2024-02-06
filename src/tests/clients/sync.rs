#[cfg(test)]
mod tests {
    use crate::api::requests::sync::Requests;
    use crate::api::responses::result::ResponseResult;
    use crate::api::types::user::User;
    use crate::tests::helpers::*;

    #[test]
    fn get_me() {
        let mock_response =  "{\"ok\":true,\"result\":{\"id\":6591790550,\"is_bot\":true,\"first_name\":\"development\",\"username\":\"development_bot\",\"can_join_groups\":true,\"can_read_all_group_messages\":false,\"supports_inline_queries\":false}}";

        let mut server = mockito::Server::new();
        let mocked_api = mock_api(&server, mocked_token(None));
        let mocked_server = mock_server(&mut server, mocked_token(None), "getMe", mock_response);

        let mock_user = serde_json::from_str::<ResponseResult<User>>(mock_response)
            .unwrap()
            .result;
        let real_user = mocked_api.sync.get_me().unwrap();

        assert_eq!(mock_user, real_user);
        mocked_server.assert();
    }
}
