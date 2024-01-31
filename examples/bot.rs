use std::error::Error;
use telegram::api::requests::sync::Requests;
use telegram::client::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let api = Client::new();

    // let params = telegram::api::params::GetUpdateParams { limit: 100, offset: 249563340, timeout: 0 };
    // let response = api.sync.get_updates(&params).unwrap();

    // let params = telegram::api::params::SetWebhookParams { url: "https://localhost.ru".to_string(), ..Default::default()};
    // let response = api.sync.set_webhook(&params).unwrap();

    // let params = telegram::api::params::DeleteWebhookParams { ..Default::default() };
    // let response = api.sync.delete_webhook(&params).unwrap();

    // let response = api.sync.get_webhook_info().unwrap();

    let response = api.sync.get_me().unwrap();

    println!("Response: {:#?}", response);

    Ok(())
}
