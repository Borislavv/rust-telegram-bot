use serde_traitobject::{Serialize, Deserialize};

// DTO interface for send request of getting messages from telegram channel
pub trait GetMessagesDtoInterface: Serialize + Deserialize {
    // offset for receving messages
    fn get_offset(&self) -> i64;
}

// DTO interface for send request of sending message into telegram channel
pub trait SendMessageDtoInterface: Serialize + Deserialize {
    // user chat id
    fn get_chat_id(&self) -> i64;
    // message to send
    fn get_message(&self) -> String;
    // parse mod for message format: markdown, html, text
    fn get_parse_mod(&self) -> String;
}
