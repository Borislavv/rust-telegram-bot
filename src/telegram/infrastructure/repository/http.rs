use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;

use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::domain::telegram::gateway::dto;
use crate::domain::telegram::gateway::contract;
use crate::domain::telegram::gateway::dto::response::{FailedSendMessageDto, SuccessSendMessageDto};
use crate::domain::telegram::gateway::r#enum::endpoint::Endpoint;

pub struct HttpRepository {
    // telegram bot url
    endpoint: String,
    // specific telegram bot token
    token: String,
    // telegram api methods
    methods: HashMap<Endpoint, String>
}

impl HttpRepository {
    pub fn new(endpoint: String, token: String) -> Self {
        let mut methods: HashMap<Endpoint, String> = HashMap::new();
            methods.insert(Endpoint::GetMessagesMethod, "getUpdates".to_string());
            methods.insert(Endpoint::SendMessageMethod, "sendMessage".to_string());

        return HttpRepository { endpoint, token, methods};
    }
}

impl contract::gateway::GatewayInterface for HttpRepository {
    fn get_messages<T>(&self, request: T) -> Result<dto::response::GetMessagesDto, String>
    where
        T: contract::request::GetMessagesDtoInterface
    {
        let method = match self.methods.get(&Endpoint::GetMessagesMethod) {
            Some(val) => val,
            None => panic!("fatal: endpoint for request messages not found into the map")
        };

        let result = Client::new()
            .get(format!("{}/bot{}/{}", self.endpoint, self.token, method))
            .query(&[("offset", request.get_offset())])
            .send();

        match result {
            Ok(response) => {
                let bytes = response.bytes().unwrap();
                let json = std::str::from_utf8(&bytes).unwrap();
                let obj: dto::response::GetMessagesDto = serde_json::from_str(json).unwrap();

                return Ok(obj);
            },
            Err(err) => Err(err.to_string())
        }
    }

    fn send_message<T>(&self, request: T) -> Result<SuccessSendMessageDto, String>
    where
        T: contract::request::SendMessageDtoInterface + Serialize + Debug
    {
        let method = match self.methods.get(&Endpoint::SendMessageMethod) {
            Some(val) => val,
            None => panic!("fatal: endpoint for send message not found into the map")
        };

        let result = Client::new()
            .post(format!("{}/bot{}/{}", self.endpoint, self.token, method))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&request).unwrap())
            .send();

        return match result {
            Ok(response) => {
                let status_code = response.status();
                let bytes = response.bytes().unwrap();
                let json = std::str::from_utf8(&bytes).unwrap();

                return match status_code {
                    StatusCode::OK => {
                        let success_resp: SuccessSendMessageDto = serde_json::from_str(json).unwrap();
                        return Ok(success_resp);
                    },
                    _ => {
                        let failed_resp: FailedSendMessageDto = serde_json::from_str(json).unwrap();
                        return Err(format!("status_code: {}, description: {}", failed_resp.error_code, failed_resp.description));
                    }
                };
            },
            Err(err) => Err(err.to_string())
        };
    }
}
