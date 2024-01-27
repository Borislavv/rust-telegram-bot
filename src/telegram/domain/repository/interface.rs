use crate::telegram::domain::{contract, model};
use crate::telegram::domain::r#enum::error::Error;

pub trait Telegram {
    // receiving messages from telegram channel
    fn get_messages(&self, request: Box<dyn contract::request::GetMessagesDtoInterface>)
        -> Result<model::response::GetMessagesDto, Error>;

    // sending message to telegram channel
    fn send_message(&self, request: Box<dyn contract::request::SendMessageDtoInterface>)
        -> Result<model::response::SuccessSendMessageDto, Error>;
}