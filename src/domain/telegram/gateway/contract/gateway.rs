use crate::domain::telegram::gateway::contract::request;
use crate::domain::telegram::gateway::{contract, dto};

use serde_traitobject::Box;

pub trait GatewayInterface {
    // receiving messages from telegram channel
    fn get_messages(&self, request: Box<dyn contract::request::GetMessagesDtoInterface>)
        -> Result<dto::response::GetMessagesDto, String>;
    // sending message to telegram channel
    fn send_message(&self, request: Box<dyn request::SendMessageDtoInterface>) 
        -> Result<dto::response::SendMessageDto, String>;
}