use std::fmt::Debug;
use serde::Serialize;

use crate::telegram::domain::model;
use crate::telegram::domain::contract;

pub trait MessageSender {
    fn send_message<T>(&self, request: T) -> Result<model::response::SuccessSendMessageDto, String>
    where
        T: contract::request::SendMessageDtoInterface + Serialize + Debug;
}