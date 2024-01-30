use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use crate::telegram::app::service::sender::sender_interface::MessageSender;
use crate::telegram::domain::{contract, model, repository};
use crate::telegram::domain::r#enum::error::Error;

pub struct Sender {
    repo: Rc<dyn repository::interface::Telegram>
}

impl Sender {
    pub fn new(repo: Rc<dyn repository::interface::Telegram>) -> Self {
        Self { repo }
    }
}

impl MessageSender for Sender {
    fn send_message(&self, request: Box<dyn contract::request::SendMessageDtoInterface>)
        -> Pin<Box<dyn Future<Output = Result<model::response::SuccessSendMessageDto, Error>> + '_>>
    {
        self.repo.send_message(request)
    }
}