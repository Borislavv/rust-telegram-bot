use std::future::Future;
use std::pin::Pin;
use crate::telegram::domain::model;
use crate::telegram::domain::contract;
use crate::telegram::domain::r#enum::error::Error;

pub trait MessageSender {
    fn send_message(&self, request: Box<dyn contract::request::SendMessageDtoInterface>)
        -> Pin<Box<dyn Future<Output = Result<model::response::SuccessSendMessageDto, Error>> + '_>>;
}