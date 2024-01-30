use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use crate::telegram::app::service::receiver::receiver_interface::MessageReceiver;
use crate::telegram::domain::{contract, model, repository};
use crate::telegram::domain::r#enum::error::Error;

pub struct Receiver {
    repo: Rc<dyn repository::interface::Telegram>
}

impl Receiver {
    pub fn new(repo: Rc<dyn repository::interface::Telegram>) -> Self {
        Self { repo }
    }
}

impl MessageReceiver for Receiver {
    fn get_messages(&self, request: Box<dyn contract::request::GetMessagesDtoInterface>)
        -> Pin<Box<dyn Future<Output = Result<model::response::GetMessagesDto, Error>> + '_>>
    {
        self.repo.get_messages(request)
    }
}