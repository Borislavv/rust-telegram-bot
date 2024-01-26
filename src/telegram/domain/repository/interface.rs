use crate::telegram::domain::{contract, model};

pub trait Telegram {
    // receiving messages from telegram channel
    fn get_messages(&self, request: Box<dyn contract::request::GetMessagesDtoInterface>)
        -> Result<model::response::GetMessagesDto, String>;

    // sending message to telegram channel
    fn send_message(&self, request: Box<dyn contract::request::SendMessageDtoInterface>)
        -> Result<model::response::SuccessSendMessageDto, String>;
}