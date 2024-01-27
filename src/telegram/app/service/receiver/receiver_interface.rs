use crate::telegram::domain::model;
use crate::telegram::domain::contract;
use crate::telegram::domain::r#enum::error::Error;

pub trait MessageReceiver {
    fn get_messages(&self, request: Box<dyn contract::request::GetMessagesDtoInterface>)
        -> Result<model::response::GetMessagesDto, Error>;
}
