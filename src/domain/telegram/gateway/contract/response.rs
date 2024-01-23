use crate::dto::response;

use serde_traitobject::{Serialize, Deserialize, Debug};

pub trait GetMessagesDtoInterface: Serialize + Deserialize + Debug {
    // is ok? - request status
    fn is_ok(&self) -> bool;
    // message struct which has data
    fn get_messages(&self) -> &Vec<response::MessageDto>;
}

pub trait MessageDtoInterface: Serialize + Deserialize + Debug  {
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

pub trait FromDtoInterface: Serialize + Deserialize + Debug {
    // user id which sent a message
    fn get_user_id(&self) -> i64;
    // is bot - identifier
    fn is_bot(&self) -> bool;
    // user first name 
    fn get_first_name(&self) -> String;
    // user last name
    fn get_last_name(&self) -> String;
    // nickname of user
    fn get_username(&self) -> String;
    // user language code
    fn get_language_code(&self) -> String;
}

pub trait ChatDtoInterface: Serialize + Deserialize + Debug {
    // user chat id
    fn get_chat_id(&self) -> i64;
    // user first name 
    fn get_first_name(&self) -> String;
    // user last name
    fn get_last_name(&self) -> String;
    // nickname of user
    fn get_username(&self) -> String;
    // chat type: private, public, channel
    fn get_type(&self) -> String;
}

pub trait SendMessageDtoInterface: Serialize + Deserialize + Debug {
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
    fn get_text(&self) -> String;
}