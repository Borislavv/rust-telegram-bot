mod telegram;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;


use std::rc::Rc;
use crate::telegram::app::facade::TelegramFacade;
use crate::telegram::app::facade_interface::TelegramFacadeInterface;
use crate::telegram::app::service::receiver::receiver::Receiver;
use crate::telegram::app::service::receiver::receiver_interface::MessageReceiver;
use crate::telegram::app::service::sender::sender::Sender;

use crate::telegram::domain::model;
use crate::telegram::domain::repository::interface::Telegram;
use crate::telegram::infrastructure::repository;

fn main() {
    let telegram_repository = Rc::new(
        repository::http::Telegram::new(
            "https://api.telegram.org".to_string(),
            "6283148707:AAE34Fob6V8cDlgzhspHKv7TX-r2CLdYXTs123".to_string()
        )
    );

    let telegram_facade: Rc<dyn TelegramFacadeInterface> = Rc::new(
        TelegramFacade::new(
            Rc::new(Sender::new(telegram_repository.clone())),
            Rc::new(Receiver::new(telegram_repository))
        )
    );

    // let result = repo.get_messages(model::request::GetMessagesDto::new(904019216));
    let get_messages_resp = telegram_facade.get_messages(Box::new(model::request::GetMessagesDto::new(0)))
        .expect("Failed to get messages. Error occurred: ");

    println!("RESPONSE OF GET MESSAGES: {:?}", get_messages_resp);

    let send_message_resp = telegram_facade.send_message(
       Box::new(model::request::SendMessageDto::new(1063099947, "Fuck yeah)))".to_string(), "html".to_string()))
    ).expect("Failed to send message. Error occurred: ");

    println!("RESPONSE OF SEND MESSAGE: {:?}", send_message_resp)
}