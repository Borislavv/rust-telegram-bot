use crate::dto::response;

pub trait GetMessagesDtoInterface {
    // is ok? - request status
    fn is_ok(&self) -> bool;
    // message struct which has data
    fn get_messages(&self) -> &Vec<response::MessageDto>;
}

pub trait MessageDtoInterface {
    // message number in queue 
    fn get_queue_id(&self) -> i64;
    // user chat id
    fn get_chat_id(&self) -> i64;
    // user first name 
    fn get_first_name(&self) -> &str;
    // user last name
    fn get_last_name(&self) -> Option<&str>;
    // nickname of user
    fn get_username(&self) -> &str;
    // user chat type: channel, private chat, public chat
    fn get_chat_type(&self) -> &str;
    // message date in unix timestamp
    fn get_date(&self) -> i64;
    // message text
    fn get_text(&self) -> &str;
}

pub trait FromDtoInterface {
    // user id which sent a message
    fn get_user_id(&self) -> i64;
    // is bot - identifier
    fn is_bot(&self) -> bool;
    // user first name 
    fn get_first_name(&self) -> &str;
    // user last name
    fn get_last_name(&self) -> Option<&str>;
    // nickname of user
    fn get_username(&self) -> &str;
    // user language code
    fn get_language_code(&self) -> Option<&str>;

    fn is_premium(&self) -> Option<bool>;
}

pub trait ChatDtoInterface {
    // user chat id
    fn get_chat_id(&self) -> i64;
    // user first name 
    fn get_first_name(&self) -> &str;
    // user last name
    fn get_last_name(&self) -> Option<&str>;
    // nickname of user
    fn get_username(&self) -> &str;
    // chat type: private, public, channel
    fn get_type(&self) -> &str;
}

pub trait SendMessageDtoInterface {
    // has been message sent successfully
    fn is_ok(&self) -> bool;
    // message external primary key
    fn get_message_id(&self) -> i64;
    // whom send a message data
    fn get_from_data(&self) -> &response::FromDto;
    // which chat is used data
    fn get_chat_data(&self) -> &response::ChatDto;
    // date in unix timestamp
    fn get_date(&self) -> i64;
    // message text
    fn get_text(&self) -> &str;
}