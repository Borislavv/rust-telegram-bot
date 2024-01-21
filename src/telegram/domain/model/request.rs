use crate::domain::telegram::gateway::contract;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMessagesDto {
    // offset for receving messages
    offset: i64
}

impl GetMessagesDto {
    pub fn new(offset: i64) -> Self {
        return GetMessagesDto { offset };
    }
}

impl contract::request::GetMessagesDtoInterface for GetMessagesDto {
    fn get_offset(&self) -> i64 {
        return self.offset;
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageDto {
    // user chat id
    chat_id: i64,
    // message to send
    text: String,
    // parse mod for message format: markdown, html, text
    parse_mod: String
}

impl SendMessageDto {
    pub fn new(chat_id: i64, text: String, parse_mod: String) -> Self {
        return SendMessageDto { chat_id, text, parse_mod };
    }
}

impl contract::request::SendMessageDtoInterface for SendMessageDto {
    fn get_chat_id(&self) -> i64 {
        return self.chat_id;
    }

    fn get_message(&self) -> &str {
        return &self.text;
    }

    fn get_parse_mod(&self) -> &str {
        return &self.parse_mod;
    }
}