#[cfg(test)]
mod tests {
    use crate::api::requests::sync::Requests;
    use crate::api::responses::error::ResponseError;
    use crate::api::types::user::User;
    use crate::errors::Error;
    use crate::tests::helpers::*;
    use std::fs;

    #[test]
    fn get_me_success() {
        let mock_response = fs::read_to_string("src/tests/responses/get_me_success.json").unwrap();
        let mut server = mockito::Server::new();
        let mocked = Mocked::new(&mut server, "getMe", &mock_response);

        let mock_user = mocked.result::<User>().unwrap();
        let real_user = mocked.client.sync.get_me().unwrap();

        assert_eq!(mock_user, real_user);
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
            assert_eq!(mock_error, real_error)
        }

        mocked.server.assert();
    }
}
