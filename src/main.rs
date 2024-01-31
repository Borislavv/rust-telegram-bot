use std::collections::HashMap;
use std::rc::Rc;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use std::sync::Arc;
use actix_web::http::header;
use crate::telegram::app::facade::TelegramFacade;
use crate::telegram::app::facade_interface::TelegramFacadeInterface;
use crate::telegram::app::service::receiver::receiver::Receiver;
use crate::telegram::app::service::sender::sender::Sender;
use crate::telegram::domain::model;
use crate::telegram::infrastructure::repository;

mod telegram;


#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    age: i64,
}

async fn perform() -> impl Responder {
    let mut outter_handlers = Vec::new();
    let inner_handlers = Arc::new(Mutex::new(Vec::with_capacity(10_000)));

    for _ in 0..10 {
        let cloned_inner_handlers = inner_handlers.clone();

        outter_handlers.push(
            tokio::spawn(async move {
                for i in 0..1_000 {
                    cloned_inner_handlers.lock().await.push(
                        tokio::spawn(async move {
                            let user = get_user(i);
                            let _ = serde_json::to_string(&user).unwrap();
                        })
                    );
                }
            })
        );
    }

    for outter_handler in outter_handlers {
        outter_handler.await;
    }

    for handler in inner_handlers.lock().await.iter_mut() {
        handler.await;
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"data\": {\"success\": true}}")
}

fn get_user(num: i64) -> User {
    User {
        name: format!("John - {}", num),
        email: format!("johnjohnson{}@gmail.com", num),
        age: num,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/v1/perform", web::get().to(perform))
            .route("/api/v1/send-and-receive", web::get().to(send_and_receive))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


async fn send_and_receive() -> impl Responder {
    let telegram_repository = Rc::new(
        repository::http::Telegram::new(
            "https://api.telegram.org".to_string(),
            "6283148707:AAE34Fob6V8cDlgzhspHKv7TX-r2CLdYXTs".to_string()
        )
    );

    let telegram_facade: Rc<dyn TelegramFacadeInterface> = Rc::new(
        TelegramFacade::new(
            Rc::new(Sender::new(telegram_repository.clone())),
            Rc::new(Receiver::new(telegram_repository))
        )
    );

    let get_messages_resp = telegram_facade
        .get_messages(Box::new(model::request::GetMessagesDto::new(0)))
        .await
        .expect("Failed to get messages. Error occurred: ");

    println!("RESPONSE OF GET MESSAGES: {:?}", get_messages_resp);

    let send_message_resp = telegram_facade
        .send_message(
            Box::new(model::request::SendMessageDto::new(1063099947, "Fuck yeah)))".to_string(), "html".to_string()))
        )
        .await
        .expect("Failed to send message. Error occurred: ");

    println!("RESPONSE OF SEND MESSAGE: {:?}", send_message_resp);

    let mut resp = HashMap::new();
    let mut data = HashMap::new();
    data.insert("success", true);
    resp.insert("data", data);
    let json = serde_json::to_string(&resp).expect("serde response serialization error");

    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .body(json)
}