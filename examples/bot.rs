use std::error::Error;
use telegram::api::response::{ResponseError, ResponseSuccess};
use telegram::api::types::User;
use telegram::Api;


fn main() -> Result<(), Box<dyn Error>> {
    let api = Api::new();

    let response = api.client
        .post(format!("{}{}", api.url, "getMe"))
        .send().unwrap();

    let body = response.text().unwrap();
    println!("JSON: {:#?}", body);

    let success: Result<ResponseSuccess<User>, serde_json::Error> = serde_json::from_str(&body);
    println!("Success: {:#?}", success);

    let error: Result<ResponseError, serde_json::Error> = serde_json::from_str(&body);
    println!("Error: {:#?}", error);

    Ok(())
}


