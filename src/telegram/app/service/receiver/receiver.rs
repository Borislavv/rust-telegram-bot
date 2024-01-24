use std::rc::Rc;
use crate::telegram::domain::{contract, model, repository};

pub struct Receiver {
    repo: Rc<dyn repository::interface::Telegram>
}

impl Receiver {
    pub fn new(repo: Rc<dyn repository::interface::Telegram>) -> Self
    {
        return Self { repo }
    }

    pub fn get_messages(&self, request: Box<dyn contract::request::GetMessagesDtoInterface>)
        -> Result<model::response::GetMessagesDto, String>
    {
        return self.repo.get_messages(request);
    }
}