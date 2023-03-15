use serde_traitobject::{Serialize, Deserialize, Box, Debug};

pub trait GetMessagesDtoInterface: Serialize + Deserialize + Debug {
    // is ok? - request status
    fn is_ok(&self) -> bool;
    // message struct which has data
    fn get_messages(&self) -> Vec<Box<dyn MessageDtoInterface>>;
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

pub trait SendMessageDtoInterface: Serialize + Deserialize + Debug {
    // has been message sent successfully
    fn is_ok(&self) -> Result<(), String>;
}