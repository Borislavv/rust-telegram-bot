use crate::domain::telegram::gateway::contract::response::{self, MessageDtoInterface};
use crate::domain::telegram::gateway::contract;

use serde_traitobject::{Box, Debug};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMessagesDto {
    // is ok? - request status
    ok: bool,
    // message struct which has data
    result: Vec<Box<MessageDto>>
}

impl GetMessagesDto {
    pub fn new(ok: bool, messages: Vec<Box<MessageDto>>) -> Self {
        return GetMessagesDto { ok, result: messages };
    }
}

impl contract::response::GetMessagesDtoInterface for GetMessagesDto {
    fn is_ok(&self) -> bool {
        return self.ok;
    }

    fn get_messages(&self) -> Vec<Box<dyn contract::response::MessageDtoInterface>> {
        let mut items: Vec<Box<dyn contract::response::MessageDtoInterface>> = vec![];

        for res in self.clone().result {
            items.push(res as Box<dyn response::MessageDtoInterface>);
        }

        return items;
    }
}

impl Clone for GetMessagesDto {
    fn clone(&self) -> Self {
        let mut messages: Vec<Box<MessageDto>> = vec![];

        for message in &self.result {
            messages.push(
                Box::new(
                    MessageDto::new(
                        message.get_queue_id(), 
                        message.get_chat_id(), 
                        message.get_first_name(), 
                        message.get_last_name(), 
                        message.get_username(), 
                        message.get_chat_type(), 
                        message.get_date(), 
                        message.get_text()
                    )
                )
            );
        }

        return GetMessagesDto { ok: self.ok, result: messages };
    }

    fn clone_from(&mut self, source: &Self)
    {
        *self = source.clone();
        drop(source);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDto {
    // message number in queue 
    update_id: i64,
    // data container
    message: DataDto
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataDto {
    // chat data
    chat: ChatDto,
    // message date in unix timestamp
    date: i64,
    // message text
    text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatDto {
    // user chat id
    id: i64,
    // user first name 
    first_name: String,
    // user last name
    last_name: String,
    // nickname of user in telegram
    username: String,
    // user chat type: channel, private chat, public chat
    r#type: String,
}

impl MessageDto {
    pub fn new(
        queue_id: i64,
        chat_id: i64,
        first_name: String,
        last_name: String,
        username: String,
        chat_type: String,
        date: i64,
        text: String
    ) -> Self {
        return MessageDto { 
            update_id: queue_id,
            message: DataDto { 
                chat: ChatDto { 
                    id: chat_id, 
                    first_name: first_name, 
                    last_name: last_name, 
                    username: username, 
                    r#type: chat_type
                }, 
                date: date, 
                text: text
            } 
        };
    }
}

impl contract::response::MessageDtoInterface for MessageDto {
    fn get_queue_id(&self) -> i64 {
        return self.update_id;
    }

    fn get_chat_id(&self) -> i64 {
        return self.message.chat.id;
    }
    
    fn get_first_name(&self) -> String {
        return self.message.chat.first_name.clone();
    }
    
    fn get_last_name(&self) -> String {
        return self.message.chat.last_name.clone();
    }
    
    fn get_username(&self) -> String {
        return self.message.chat.username.clone();
    }
    
    fn get_chat_type(&self) -> String {
        return self.message.chat.r#type.clone();
    }

    fn get_date(&self) -> i64 {
        return self.message.date;
    }
    
    fn get_text(&self) -> String {
        return self.message.text.clone();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageDto {
    result: Result<(), String>
}

impl SendMessageDto {
    pub fn new(result: Result<(), String>) -> Self {
        return SendMessageDto { result };
    }
}

impl contract::response::SendMessageDtoInterface for SendMessageDto {
    fn is_ok(&self) -> Result<(), String> {
        return self.result.clone();
    }
}