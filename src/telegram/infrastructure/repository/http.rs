use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use reqwest::{Client, StatusCode};

use crate::telegram::domain::repository;
use crate::telegram::domain::model;
use crate::telegram::domain::contract;
use crate::telegram::domain::model::response::{FailedResponseDto, GetMessagesDto, SuccessSendMessageDto};
use crate::telegram::domain::r#enum::endpoint::Endpoint;
use crate::telegram::domain::r#enum::error::Error;

pub struct Telegram {
    // telegram bot url
    endpoint: String,
    // specific telegram bot token
    token: String,
}

impl Telegram {
    pub fn new(endpoint: String, token: String) -> Self {
        return Telegram { endpoint, token };
    }
}

impl repository::interface::Telegram for Telegram
{
    fn get_messages(&self, request: Box<dyn contract::request::GetMessagesDtoInterface>)
        -> Pin<Box<dyn Future<Output = Result<model::response::GetMessagesDto, Error>> + '_>>
    {
        let url = format!("{}/bot{}/{}", self.endpoint, self.token, Endpoint::GetMessagesMethod.as_str());

        Box::pin(
            async move {
                let response = Client::new()
                    .get(url)
                    .query(&[("offset", request.get_offset())])
                    .send()
                    .await?;

                let status_code = response.status();
                let bytes = response.bytes().await?;
                let json = std::str::from_utf8(&bytes)?;

                println!("{}", json);

                match status_code {
                    StatusCode::OK => Ok(serde_json::from_str::<GetMessagesDto>(json)?),
                    _ => Err(Error::TelegramError(serde_json::from_str::<FailedResponseDto>(json)?.description)),
                }
            }
        )
    }

    fn send_message(&self, request: Box<dyn contract::request::SendMessageDtoInterface>)
        -> Pin<Box<dyn Future<Output = Result<model::response::SuccessSendMessageDto, Error>> + '_>>
    {
        let url = format!("{}/bot{}/{}", self.endpoint, self.token, Endpoint::SendMessageMethod.as_str());

        Box::pin(
            async move {
                let chat_id = request.get_chat_id().to_string();
                let mut body: HashMap<&str, &str> = HashMap::new();
                body.insert("text", request.get_message());
                body.insert("chat_id", chat_id.as_str());
                body.insert("parse_mod", request.get_parse_mod());

                let response = Client::new()
                    .post(url)
                    .header(reqwest::header::CONTENT_TYPE, "application/json")
                    .body(serde_json::to_string(&body)?)
                    .send()
                    .await?;

                let status_code = response.status();
                let bytes = response.bytes().await?;
                let json = std::str::from_utf8(&bytes)?;

                match status_code {
                    StatusCode::OK => Ok(serde_json::from_str::<SuccessSendMessageDto>(json)?),
                    _ => Err(Error::TelegramError(serde_json::from_str::<FailedResponseDto>(json)?.description)),
                }
            }
        )
    }
}
