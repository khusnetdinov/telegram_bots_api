use crate::api::params::copy_message::CopyMessage;
use crate::api::params::copy_messages::CopyMessages;
use crate::api::params::delete_webhook::DeleteWebhook;
use crate::api::params::forward_message::ForwardMessage;
use crate::api::params::forward_messages::ForwardMessages;
use crate::api::params::get_update::GetUpdate;
use crate::api::params::send_message::SendMessage;
use crate::api::params::set_webhook::SetWebhook;
use crate::api::requests::sync::Requests;
use crate::api::responses::error::ResponseError;
use crate::api::responses::result::ResponseResult;
use crate::api::types::message::Message;
use crate::api::types::message_id::MessageId;
use crate::api::types::update::Update;
use crate::api::types::user::User;
use crate::api::types::webhook_info::WebhookInfo;
use crate::clients::traits::{Decoder, Responder};
use crate::config::Config;
use crate::errors::Error;
use reqwest::blocking::{ClientBuilder, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use std::time::Duration;

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
    fn get_updates(&self, params: &GetUpdate) -> Result<Vec<Update>, Error> {
        self.respond_with::<Vec<Update>>(self.request_for("getUpdates").json(params).send())
    }

    fn set_webhook(&self, params: &SetWebhook) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request_for("setWebhook").json(params).send())
    }

    fn delete_webhook(&self, params: &DeleteWebhook) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request_for("deleteWebhook").json(params).send())
    }

    fn get_webhook_info(&self) -> Result<WebhookInfo, Error> {
        self.respond_with::<WebhookInfo>(self.request_for("getWebhookInfo").json(&{}).send())
    }

    fn get_me(&self) -> Result<User, Error> {
        self.respond_with::<User>(self.request_for("getMe").json(&{}).send())
    }

    fn log_out(&self) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request_for("logOut").json(&{}).send())
    }

    fn close(&self) -> Result<bool, Error> {
        self.respond_with::<bool>(self.request_for("close").json(&{}).send())
    }

    fn send_message(&self, params: &SendMessage) -> Result<Message, Error> {
        self.respond_with::<Message>(self.request_for("sendMessage").json(params).send())
    }

    fn forward_message(&self, params: &ForwardMessage) -> Result<MessageId, Error> {
        self.respond_with::<MessageId>(self.request_for("forwardMessage").json(params).send())
    }

    fn forward_messages(&self, params: &ForwardMessages) -> Result<Vec<MessageId>, Error> {
        self.respond_with::<Vec<MessageId>>(self.request_for("forwardMessages").json(params).send())
    }

    fn copy_message(&self, params: &CopyMessage) -> Result<MessageId, Error> {
        self.respond_with::<MessageId>(self.request_for("copyMessage").json(params).send())
    }

    fn copy_messages(&self, params: &CopyMessages) -> Result<Vec<MessageId>, Error> {
        self.respond_with::<Vec<MessageId>>(self.request_for("copyMessages").json(params).send())
    }
}
