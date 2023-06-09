use crate::domain::telegram::gateway::contract::response::{self, MessageDtoInterface};
use crate::domain::telegram::gateway::contract;

use serde_traitobject::Box;

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
    // user id
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

impl Clone for ChatDto {
    fn clone(&self) -> Self {
        return ChatDto { 
            id: self.id, 
            first_name: self.first_name.clone(), 
            last_name: self.last_name.clone(), 
            username: self.username.clone(), 
            r#type: self.r#type.clone()
        };
    }

    fn clone_from(&mut self, source: &Self)
    {
        *self = source.clone();
        drop(source);
    }
}

impl contract::response::ChatDtoInterface for ChatDto {
    fn get_chat_id(&self) -> i64 {
        return self.id;
    }

    fn get_first_name(&self) -> String {
        return self.first_name.clone();
    }

    fn get_last_name(&self) -> String {
        return self.last_name.clone();
    }

    fn get_username(&self) -> String {
        return self.username.clone();
    }
    
    fn get_type(&self) -> String {
        return self.r#type.clone();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FromDto {
    // user id
    id: i64,
    // is bot - indicator
    is_bot: bool,
    // user first name 
    first_name: String,
    // user last name
    last_name: String,
    // nickname of user in telegram
    username: String,
    // user language
    language_code: String,
    // is premium user - indicator
    is_premium: bool
}

impl Clone for FromDto {
    fn clone(&self) -> Self {
        return FromDto { 
            id: self.id, 
            is_bot: self.is_bot, 
            first_name: self.first_name.clone(), 
            last_name: self.last_name.clone(), 
            username: self.username.clone(), 
            language_code: self.language_code.clone(), 
            is_premium: self.is_premium
        };
    }

    fn clone_from(&mut self, source: &Self)
    {
        *self = source.clone();
        drop(source);
    }
}

impl contract::response::FromDtoInterface for FromDto {
    fn get_user_id(&self) -> i64 {
        return self.id;
    }

    fn get_first_name(&self) -> String {
        return self.first_name.clone();
    }

    fn get_last_name(&self) -> String {
        return self.last_name.clone();
    }

    fn get_username(&self) -> String {
        return self.username.clone();
    }

    fn get_language_code(&self) -> String {
        return self.language_code.clone();
    }

    fn is_bot(&self) -> bool {
        return self.is_bot;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageDto {
    // is ok? - request status
    ok: bool,
    // message struct which has data
    result: ResultDto
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultDto {
    // external message primary key
    message_id: i64,
    // whom sent a message
    from: FromDto,
    // which chat was used
    chat: ChatDto,
    // unix timestamp
    date: i64,
    // message text
    text: String

}

impl contract::response::SendMessageDtoInterface for SendMessageDto {
    fn is_ok(&self) -> bool {
        return self.ok;
    }

    fn get_message_id(&self) -> i64 {
        return self.result.message_id;
    }

    fn get_from_data(&self) -> Box<dyn contract::response::FromDtoInterface> {
        return self.clone().get_from_data();
    }

    fn get_chat_data(&self) -> Box<dyn contract::response::ChatDtoInterface> {
        return self.clone().get_chat_data();
    }

    fn get_date(&self) -> i64 {
        return self.result.date;
    }

    fn get_text(&self) -> String {
        return self.result.text.clone();
    }
}