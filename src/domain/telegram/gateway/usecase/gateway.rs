use std::collections::HashMap;

use reqwest::blocking::Client;

use crate::domain::telegram::gateway::dto;
use crate::domain::telegram::gateway::contract;
use crate::domain::telegram::gateway::r#enum::endpoint::Endpoint;

use serde_traitobject::Box;

pub struct Gateway {
    // telegram bot url
    endpoint: String,
    // specific telegram bot token
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
    fn get_messages(&self, request: Box<dyn contract::request::GetMessagesDtoInterface>)
        -> Result<dto::response::GetMessagesDto, String> {

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

    fn send_message(&self, request: Box<dyn contract::request::SendMessageDtoInterface>)
            -> Result<dto::response::SendMessageDto, String> {

        let method = match self.methods.get(&Endpoint::SendMessageMethod) {
            Some(val) => val,
            None => panic!("fatal: endpoint for send message not found into the map")
        };

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

                return Ok(obj);
            },
            Err(err) => Err(err.to_string())
        }
    }
}
