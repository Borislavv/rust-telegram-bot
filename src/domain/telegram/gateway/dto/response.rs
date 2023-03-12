use std::fmt::Error;

// 
pub trait GetMessagesResponseDtoInterface {
    // message struct which has data
    fn get_messages(&self) -> &Vec<Box<dyn MessageResponseDtoInterface>>;
}

pub trait MessageResponseDtoInterface {
    // message number in queue 
    fn get_queue_id(&self) -> i64;
    // user chat id
    fn get_chat_id(&self) -> i64;
    // user first name 
    fn get_first_name(&self) -> String;
    // user last name
    fn get_last_name(&self) -> String;
    // nickname of user
    fn get_username(&self) -> String;
    // user chat type: channel, private chat, public chat
    fn get_chat_type(&self) -> String;
    // message date in unix timestamp
    fn get_date(&self) -> i64;
    // message text
    fn get_text(&self) -> String;
}

pub trait SendMessageResponseDtoInterface {
    // has been message sent successfully
    fn is_ok(&self) -> Result<(), Error>;
}

pub struct GetMessagesResponseDto {
    // message struct which has data
    messages: Vec<Box<dyn MessageResponseDtoInterface>>
}

impl GetMessagesResponseDto {
    pub fn new(messages: Vec<Box<dyn MessageResponseDtoInterface>>) -> Self {
        return GetMessagesResponseDto { messages };
    }
}

impl GetMessagesResponseDtoInterface for GetMessagesResponseDto {
    fn get_messages(&self) -> &Vec<Box<dyn MessageResponseDtoInterface>> {
        return &self.messages;
    }
}

pub struct MessageResponseDto {
    // message number in queue 
    queue_id: i64,
    // user chat id
    chat_id: i64,
    // user first name 
    firstname: String,
    // user last name
    lastname: String,
    // nickname of user
    username: String,
    // user chat type: channel, private chat, public chat
    chat_type: String,
    // message date in unix timestamp
    date: i64,
    // message text
    text: String
}

impl MessageResponseDto {
    pub fn new(
        queue_id: i64,
        chat_id: i64,
        firstname: String,
        lastname: String,
        username: String,
        chat_type: String,
        date: i64,
        text: String
    ) -> Self {
        return MessageResponseDto { 
            queue_id, 
            chat_id, 
            firstname, 
            lastname, 
            username, 
            chat_type, 
            date, 
            text 
        };
    }
}

impl MessageResponseDtoInterface for MessageResponseDto {
    fn get_queue_id(&self) -> i64 {
        return self.queue_id;
    }

    fn get_chat_id(&self) -> i64 {
        return self.chat_id;
    }
    
    fn get_first_name(&self) -> String {
        return self.firstname.clone();
    }
    
    fn get_last_name(&self) -> String {
        return self.lastname.clone();
    }
    
    fn get_username(&self) -> String {
        return self.username.clone();
    }
    
    fn get_chat_type(&self) -> String {
        return self.chat_type.clone();
    }

    fn get_date(&self) -> i64 {
        return self.date;
    }
    
    fn get_text(&self) -> String {
        return self.text.clone();
    }
}

pub struct SendMessageRequestDto {
    result: Result<(), Error>
}

impl SendMessageRequestDto {
    pub fn new(result: Result<(), Error>) -> Self {
        return SendMessageRequestDto { result };
    }
}

impl SendMessageResponseDtoInterface for SendMessageRequestDto {
    fn is_ok(&self) -> Result<(), Error> {
        return self.result;
    }
}