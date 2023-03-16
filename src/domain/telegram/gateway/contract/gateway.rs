use crate::domain::telegram::gateway::contract::request;
use crate::domain::telegram::gateway::contract::response;

use serde_traitobject::Box;

pub trait GatewayInterface {
    // receving messages from telegram channel
    fn get_messages(&self, request: Box<dyn request::GetMessagesDtoInterface>) 
        -> Result<Box<dyn response::GetMessagesDtoInterface>, String>;
    // sending message to telegram channel
    fn send_message(&self, request: Box<dyn request::SendMessageDtoInterface>) 
        -> Result<Box<dyn response::SendMessageDtoInterface>, String>;
}