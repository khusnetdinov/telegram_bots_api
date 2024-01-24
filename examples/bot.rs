use std::error::Error;
use std::time::Duration;
use telegram::api::response::{ResponseError, ResponseSuccess};
use telegram::api::types::User;
use telegram::Api;


fn main() -> Result<(), Box<dyn Error>> {
    let api = Api::new();

    let builder = reqwest::blocking::ClientBuilder::new();
    let client = builder
        .timeout(Duration::from_secs(api.config.timeout))
        .connect_timeout(Duration::from_secs(api.config.connect_timeout))
        .build()
        .unwrap();

    let response = client
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


