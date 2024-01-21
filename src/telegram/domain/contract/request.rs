// DTO interface for send request of getting messages from telegram channel
pub trait GetMessagesDtoInterface {
    // offset for receiving messages
    fn get_offset(&self) -> i64;
}

// DTO interface for send request of sending message into telegram channel
pub trait SendMessageDtoInterface {
    // user chat id
    fn get_chat_id(&self) -> i64;
    // message to send
    fn get_message(&self) -> &str;
    // parse mod for message format: markdown, html, text
    fn get_parse_mod(&self) -> &str;
}
