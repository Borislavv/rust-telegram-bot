use std::future::Future;
use std::pin::Pin;
use crate::telegram::domain::{contract, model};
use crate::telegram::domain::r#enum::error::Error;

pub trait Telegram {
    // receiving messages from telegram channel
    fn get_messages(&self, request: Box<dyn contract::request::GetMessagesDtoInterface>)
        -> Pin<Box<dyn Future<Output = Result<model::response::GetMessagesDto, Error>> + '_>>;

    // sending message to telegram channel
    fn send_message(&self, request: Box<dyn contract::request::SendMessageDtoInterface>)
        -> Pin<Box<dyn Future<Output = Result<model::response::SuccessSendMessageDto, Error>> + '_>>;
}