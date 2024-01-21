mod telegram;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use core::panic;

use crate::telegram::domain::model;
use crate::telegram::domain::repository::interface::Telegram;
use crate::telegram::infrastructure::repository;

fn main() {
    let repo = repository::http::Telegram::new(
        "https://api.telegram.org".to_string(),
        "6283148707:AAE34Fob6V8cDlgzhspHKv7TX-r2CLdYXTs".to_string()
    );

    // let result = repo.get_messages(model::request::GetMessagesDto::new(904019216));
    let result = repo.get_messages(Box::new(model::request::GetMessagesDto::new(0)));

    let response = match result {
        Ok(v) => v,
        Err(err) => panic!("{}", err)
    };

    println!("RESPONSE OF GET MESSAGES: {:?}", response);

    let send_message_resp = repo.send_message(
       Box::new(model::request::SendMessageDto::new(1063099947, "Fuck yeah)))".to_string(), "html".to_string()))
    );

    let send_message_resp = match send_message_resp {
        Ok(v) => v,
        Err(err) => panic!("{}", err)
    };

    println!("RESPONSE OF SEND MESSAGE: {:?}", send_message_resp)
}