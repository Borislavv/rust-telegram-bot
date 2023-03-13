use crate::domain::telegram::gateway::contract::response;

use serde_traitobject::Box;

#[derive(Serialize, Deserialize)]
pub struct GetMessagesDto {
    // message struct which has data
    // #[serde(with = "serde_traitobject")]
    messages: Vec<Box<dyn response::MessageDtoInterface>>
}

impl GetMessagesDto {
    pub fn new(messages: Vec<Box<dyn response::MessageDtoInterface>>) -> Self {
        return GetMessagesDto { messages };
    }
}

impl response::GetMessagesDtoInterface for GetMessagesDto {
    fn get_messages(&self) -> Vec<Box<dyn response::MessageDtoInterface>> {
        return self.clone().messages;
    }
}

impl Clone for GetMessagesDto {
    fn clone(&self) -> Self {
        let mut messages: Vec<Box<dyn response::MessageDtoInterface>> = vec![];

        for message in &self.messages {
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

        return GetMessagesDto { messages };
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
    data: DataDto
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
            data: DataDto { 
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

impl response::MessageDtoInterface for MessageDto {
    fn get_queue_id(&self) -> i64 {
        return self.update_id;
    }

    fn get_chat_id(&self) -> i64 {
        return self.data.chat.id;
    }
    
    fn get_first_name(&self) -> String {
        return self.data.chat.first_name.clone();
    }
    
    fn get_last_name(&self) -> String {
        return self.data.chat.last_name.clone();
    }
    
    fn get_username(&self) -> String {
        return self.data.chat.username.clone();
    }
    
    fn get_chat_type(&self) -> String {
        return self.data.chat.r#type.clone();
    }

    fn get_date(&self) -> i64 {
        return self.data.date;
    }
    
    fn get_text(&self) -> String {
        return self.data.text.clone();
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

impl response::SendMessageDtoInterface for SendMessageDto {
    fn is_ok(&self) -> Result<(), String> {
        return self.result.clone();
    }
}