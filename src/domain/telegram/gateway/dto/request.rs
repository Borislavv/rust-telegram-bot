// DTO interface for send request of getting messages from telegram channel
pub trait GetMessagesRequestDtoInterface {
    // offset for receving messages
    fn get_offset(&self) -> i64;
}

// DTO interface for send request of sending message into telegram channel
pub trait SendMessageRequestDtoInterface {
    // user chat id
    fn get_chat_id(&self) -> i64;
    // message to send
    fn get_message(&self) -> String;
    // parse mod for message format: markdown, html, text
    fn get_parse_mod(&self) -> String;
}

#[derive(Debug)]
pub struct GetMessagesRequestDto {
    // offset for receving messages
    offset: i64
}

impl GetMessagesRequestDto {
    pub fn new(offset: i64) -> Self {
        return GetMessagesRequestDto { offset };
    }
}

impl GetMessagesRequestDtoInterface for GetMessagesRequestDto {
    fn get_offset(&self) -> i64 {
        return self.offset;
    }
}


#[derive(Debug)]
pub struct SendMessageRequestDto {
    // user chat id
    chat_id: i64,
    // message to send
    message: String,
    // parse mod for message format: markdown, html, text
    parse_mod: String
}

impl SendMessageRequestDto {
    pub fn new(chat_id: i64, message: String, parse_mod: String) -> Self {
        return SendMessageRequestDto { chat_id, message, parse_mod };
    }
}

impl SendMessageRequestDtoInterface for SendMessageRequestDto {
    fn get_chat_id(&self) -> i64 {
        return self.chat_id;
    }

    fn get_message(&self) -> String {
        return self.message.clone();
    }

    fn get_parse_mod(&self) -> String {
        return self.parse_mod.clone();
    }
}