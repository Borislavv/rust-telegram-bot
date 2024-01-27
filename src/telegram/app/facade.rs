use std::rc::Rc;
use crate::telegram::app::facade_interface::TelegramFacadeInterface;
use crate::telegram::app::service::receiver::receiver_interface::MessageReceiver;
use crate::telegram::app::service::sender::sender_interface::MessageSender;
use crate::telegram::domain::contract::request::{GetMessagesDtoInterface, SendMessageDtoInterface};
use crate::telegram::domain::model::response::{GetMessagesDto, SuccessSendMessageDto};
use crate::telegram::domain::r#enum::error::Error;

pub struct TelegramFacade {
    sender: Rc<dyn MessageSender>,
    receiver: Rc<dyn MessageReceiver>
}

impl TelegramFacade {
    pub fn new(sender: Rc<dyn MessageSender>, receiver: Rc<dyn MessageReceiver>) -> Self {
        Self { sender, receiver }
    }
}

impl TelegramFacadeInterface for TelegramFacade {
}

impl MessageSender for TelegramFacade {
    fn send_message(&self, request: Box<dyn SendMessageDtoInterface>) -> Result<SuccessSendMessageDto, Error> {
        self.sender.send_message(request)
    }
}

impl MessageReceiver for TelegramFacade {
    fn get_messages(&self, request: Box<dyn GetMessagesDtoInterface>) -> Result<GetMessagesDto, Error> {
        self.receiver.get_messages(request)
    }
}