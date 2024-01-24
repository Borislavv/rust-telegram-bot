use crate::telegram::domain::model;
use crate::telegram::domain::contract;

pub trait MessageReceiver {
    fn get_messages<T>(&self, request: T) -> Result<model::response::GetMessagesDto, String>
    where
        T: contract::request::GetMessagesDtoInterface;
}
