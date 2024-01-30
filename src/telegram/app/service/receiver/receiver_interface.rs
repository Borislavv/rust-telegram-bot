use std::future::Future;
use std::pin::Pin;
use crate::telegram::domain::model;
use crate::telegram::domain::contract;
use crate::telegram::domain::r#enum::error::Error;

pub trait MessageReceiver {
    fn get_messages(&self, request: Box<dyn contract::request::GetMessagesDtoInterface>)
        -> Pin<Box<dyn Future<Output = Result<model::response::GetMessagesDto, Error>> + '_>>;
}
