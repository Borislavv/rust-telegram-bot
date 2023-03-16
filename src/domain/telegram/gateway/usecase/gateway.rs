use std::collections::HashMap;

use reqwest::blocking::Client;

use crate::domain::telegram::gateway::dto;
use crate::domain::telegram::gateway::contract;
use crate::domain::telegram::gateway::r#enum::endpoint::Endpoint;

use serde_traitobject::Box;

pub struct Gateway {
    // telegram bot url
    endpoint: String,
    // spcific telegram bot token
    token: String,
    // telegram api methods
    methods: HashMap<Endpoint, String>
}

impl Gateway {
    pub fn new(endpoint: String, token: String) -> Self {
        let mut methods: HashMap<Endpoint, String> = HashMap::new();
            methods.insert(Endpoint::GetMessagesMethod, "getUpdates".to_string());
            methods.insert(Endpoint::SendMessageMethod, "sendMessage".to_string());

        return Gateway{ endpoint, token, methods};
    }
}

impl contract::gateway::GatewayInterface for Gateway {
    // getting messages from telegram channel by given offset
    fn get_messages(&self, request: Box<dyn contract::request::GetMessagesDtoInterface>)
        -> Result<Box<dyn contract::response::GetMessagesDtoInterface>, String> {

        let method: String = (*self.methods.get(&Endpoint::GetMessagesMethod).unwrap().clone()).to_string();

        let result = Client::new()
            .get(format!("{}/bot{}/{}", self.endpoint, self.token, method))
            .query(&[("offset", request.get_offset())])
            .send();

        match result {
            Ok(response) => {
                let bytes = response.bytes().unwrap();
                let json = std::str::from_utf8(&bytes).unwrap();
                let obj: dto::response::GetMessagesDto = serde_json::from_str(json).unwrap();
                let boxed_obj = Box::new(obj) as Box<dyn contract::response::GetMessagesDtoInterface>;

                return Ok(boxed_obj);
            },
            Err(err) => Err(err.to_string())
        }
    }

    // sending message to telegram channel
    fn send_message(&self, request: Box<dyn contract::request::SendMessageDtoInterface>) 
            -> Result<Box<dyn contract::response::SendMessageDtoInterface>, String> {

        let method: String = (*self.methods.get(&Endpoint::SendMessageMethod).unwrap().clone()).to_string();

        println!("{}", serde_json::to_string(&request).unwrap());

        let result = Client::new()
            .post(format!("{}/bot{}/{}", self.endpoint, self.token, method))
            .body(serde_json::to_string(&request).unwrap())
            .send();

        match result {
            Ok(response) => {
                let bytes = response.bytes().unwrap();
                let json = std::str::from_utf8(&bytes).unwrap();

                println!("{}", json);

                let obj: dto::response::SendMessageDto = serde_json::from_str(json).unwrap();
                let boxed_obj = Box::new(obj) as Box<dyn contract::response::SendMessageDtoInterface>;

                return Ok(boxed_obj);
            },
            Err(err) => Err(err.to_string())
        }
    }
}