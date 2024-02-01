use crate::api::params::{DeleteWebhookParams, ForwardMessageParams, ForwardMessagesParams, GetUpdateParams, SendMessageParams, SetWebhookParams};
use crate::api::requests::sync::Requests;
use crate::api::responses::{ResponseError, ResponseResult};
use crate::api::types::message::Message;
use crate::api::types::update::Update;
use crate::api::types::user::User;
use crate::api::types::webhook_info::WebhookInfo;
use crate::clients::traits::{Decoder, Responder};
use crate::config::Config;
use crate::errors::Error;
use reqwest::blocking::{ClientBuilder, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use std::time::Duration;
use crate::api::types::message_id::MessageId;

#[derive(Debug)]
pub struct Sync {
    client: reqwest::blocking::Client,
    offset: i64,
    limit: i64,
    timeout: u64,
    url: String,
}

impl Sync {
    pub fn new(config: &Config) -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .build()
            .unwrap();

        let offset = config.updates_offset;
        let limit = config.updates_limit;
        let timeout = config.updates_timeout;
        let url = config.build_url();

        Self {
            client,
            offset,
            limit,
            timeout,
            url,
        }
    }

    pub fn request_for(&self, method: &str) -> RequestBuilder {
        self.client.post(format!("{}{}", self.url, method))
    }
}

impl Decoder for Sync {
    fn decode<T: DeserializeOwned>(&self, response: Response) -> Result<T, Error> {
        match serde_json::from_str::<ResponseResult<T>>(&response.text().unwrap()) {
            Ok(success) => Ok(success.result),
            Err(error) => Err(Error::Decode(error)),
        }
    }
}

impl Responder for Sync {
    fn respond_with<T: DeserializeOwned>(
        &self,
        response: Result<Response, reqwest::Error>,
    ) -> Result<T, Error> {
        match response {
            Ok(response) => match response.status().as_u16() {
                200 => self.decode::<T>(response),
                _ => Err(Error::Response(ResponseError::new(
                    &response.text().unwrap(),
                ))),
            },
            Err(error) => Err(Error::Request(error)),
        }
    }
}

impl Requests for Sync {
    fn get_updates(&self, params: &GetUpdateParams) -> Result<Vec<Update>, Error> {
        let request = self.request_for("getUpdates").json(params);

        self.respond_with::<Vec<Update>>(request.send())
    }

    fn set_webhook(&self, params: &SetWebhookParams) -> Result<bool, Error> {
        let request = self.request_for("setWebhook").json(params);

        self.respond_with::<bool>(request.send())
    }

    fn delete_webhook(&self, params: &DeleteWebhookParams) -> Result<bool, Error> {
        let request = self.request_for("deleteWebhook").json(params);

        self.respond_with::<bool>(request.send())
    }

    fn get_webhook_info(&self) -> Result<WebhookInfo, Error> {
        let request = self.request_for("getWebhookInfo").json(&{});

        self.respond_with::<WebhookInfo>(request.send())
    }

    fn get_me(&self) -> Result<User, Error> {
        let request = self.request_for("getMe").json(&{});

        self.respond_with::<User>(request.send())
    }

    fn log_out(&self) -> Result<bool, Error> {
        let request = self.request_for("logOut").json(&{});

        self.respond_with::<bool>(request.send())
    }

    fn close(&self) -> Result<bool, Error> {
        let request = self.request_for("close").json(&{});

        self.respond_with::<bool>(request.send())
    }

    fn send_message(&self, params: &SendMessageParams) -> Result<Message, Error> {
        let request = self.request_for("sendMessage").json(params);

        self.respond_with::<Message>(request.send())
    }

    fn forward_message(&self, params: &ForwardMessageParams) -> Result<MessageId, Error> {
        let request = self.request_for("forwardMessage").json(params);

        self.respond_with::<MessageId>(request.send())
    }

    fn forward_messages(&self, params: &ForwardMessagesParams) -> Result<MessageId, Error> {
        let request = self.request_for("sendMessage").json(params);

        self.respond_with::<MessageId>(request.send())
    }
}
