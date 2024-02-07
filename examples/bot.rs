use std::error::Error;
use telegram::api::requests::sync::Requests;
use telegram::client::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let api = Client::new();
    let response = api.sync.get_me();

    // let params = GetUpdate {
    //     limit: 100,
    //     offset: 249563340,
    //     timeout: 0,
    // };
    // let response = api.sync.get_updates(&params);

    // let params = SetWebhook {
    //     url: "https://localhost.ru".to_string(),
    //     ..Default::default()
    // };
    // let response = api.sync.set_webhook(&params);

    // let params = DeleteWebhook {
    //     ..Default::default()
    // };
    // let response = api.sync.delete_webhook(&params);
    // let response = api.sync.get_webhook_info();

    // let response = api.sync.log_out();
    // let response = api.sync.close();

    // let params = SendMessage {
    //     chat_id: 147951145,
    //     text: "Hello World".to_string(),
    //     ..Default::default()
    // };
    // let response = api.sync.send_message(&params);

    // let params = ForwardMessage {
    //     message_id: 456,
    //     chat_id: 147951145,
    //     from_chat_id: 147951145,
    //     ..Default::default()
    // };
    // let response = api.sync.forward_message(&params);

    // let params = ForwardMessages {
    //     message_ids: vec![454, 456],
    //     chat_id:  147951145,
    //     from_chat_id:  147951145,
    //     ..Default::default()
    // };
    // let response = api.sync.forward_messages(&params);

    println!("Response: {:#?}", response.unwrap());
    Ok(())
}
