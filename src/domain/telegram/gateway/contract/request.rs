use serde_traitobject::{Serialize, Deserialize, Debug};

// DTO interface for send request of getting messages from telegram channel
pub trait GetMessagesDtoInterface: Serialize + Deserialize + Debug {
    // offset for receiving messages
    fn get_offset(&self) -> i64;
}

// DTO interface for send request of sending message into telegram channel
pub trait SendMessageDtoInterface: Serialize + Deserialize + Debug {
    // user chat id
    fn get_chat_id(&self) -> i64;
    // message to send
    fn get_message(&self) -> String;
    // parse mod for message format: markdown, html, text
    fn get_parse_mod(&self) -> String;
}
