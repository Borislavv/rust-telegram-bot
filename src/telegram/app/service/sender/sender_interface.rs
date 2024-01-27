


use crate::telegram::domain::model;
use crate::telegram::domain::contract;
use crate::telegram::domain::r#enum::error::Error;

pub trait MessageSender {
    fn send_message(&self, request: Box<dyn contract::request::SendMessageDtoInterface>)
       -> Result<model::response::SuccessSendMessageDto, Error>;
}