use std::collections::HashMap;

use reqwest::blocking::Client;
use reqwest::StatusCode;

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
    // telegram api methods
    methods: HashMap<Endpoint, String>
}

impl Telegram {
    pub fn new(endpoint: String, token: String) -> Self {
        let mut methods: HashMap<Endpoint, String> = HashMap::new();
            methods.insert(Endpoint::GetMessagesMethod, "getUpdates".to_string());
            methods.insert(Endpoint::SendMessageMethod, "sendMessage".to_string());

        return Telegram { endpoint, token, methods};
    }
}

impl repository::interface::Telegram for Telegram
{
    fn get_messages(&self, request: Box<dyn contract::request::GetMessagesDtoInterface>)
        -> Result<model::response::GetMessagesDto, Error>
    {
        let method = match self.methods.get(&Endpoint::GetMessagesMethod) {
            Some(val) => val,
            None => panic!("fatal: endpoint for request messages not found into the map")
        };

        let response = Client::new()
            .get(format!("{}/bot{}/{}", self.endpoint, self.token, method))
            .query(&[("offset", request.get_offset())])
            .send()?;

        let status_code = response.status();
        let bytes = response.bytes()?;
        let json = std::str::from_utf8(&bytes)?;

        println!("{}", json);

        return match status_code {
            StatusCode::OK => Ok(serde_json::from_str::<GetMessagesDto>(json)?),
            _ => Err(Error::TelegramError(serde_json::from_str::<FailedResponseDto>(json)?.description)),
        };
    }

    fn send_message(&self, request: Box<dyn contract::request::SendMessageDtoInterface>)
        -> Result<model::response::SuccessSendMessageDto, Error>
    {
        let method = match self.methods.get(&Endpoint::SendMessageMethod) {
            Some(val) => val,
            None => panic!("fatal: endpoint for send message not found into the map")
        };

        let chat_id = &request.get_chat_id().to_string();
        let mut body: HashMap<&str, &str> = HashMap::new();
        body.insert("text", request.get_message());
        body.insert("chat_id", chat_id);
        body.insert("parse_mod", request.get_parse_mod());

        let response = Client::new()
            .post(format!("{}/bot{}/{}", self.endpoint, self.token, method))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&body)?)
            .send()?;

        let status_code = response.status();
        let bytes = response.bytes().unwrap();
        let json = std::str::from_utf8(&bytes)?;

        return match status_code {
            StatusCode::OK => Ok(serde_json::from_str::<SuccessSendMessageDto>(json)?),
            _ => Err(Error::TelegramError(serde_json::from_str::<FailedResponseDto>(json)?.description)),
        };
    }
}
