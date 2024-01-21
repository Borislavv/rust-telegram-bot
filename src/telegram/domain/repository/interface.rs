use std::fmt::Debug;
use serde::Serialize;
use crate::telegram::domain::{contract, model};

pub trait RepositoryInterface {
    // receiving messages from telegram channel
    fn get_messages<T>(&self, request: T) -> Result<model::response::GetMessagesDto, String>
        where
            T: contract::request::GetMessagesDtoInterface;

    // sending message to telegram channel
    fn send_message<T>(&self, request: T) -> Result<model::response::SuccessSendMessageDto, String>
        where
            T: contract::request::SendMessageDtoInterface + Serialize + Debug;
}