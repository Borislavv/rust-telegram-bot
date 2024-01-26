use crate::telegram::domain::contract;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMessagesDto {
    // is ok? - request status
    ok: bool,
    // message struct which has data
    result: Vec<MessageDto>
}

impl GetMessagesDto {
    pub fn new(ok: bool, messages: Vec<MessageDto>) -> Self {
        return GetMessagesDto { ok, result: messages };
    }
}

impl contract::response::GetMessagesDtoInterface for GetMessagesDto {
    fn is_ok(&self) -> bool {
        return self.ok;
    }

    fn get_messages(&self) -> &Vec<MessageDto> {
        return &self.result;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDto {
    // message number in queue 
    update_id: i64,
    // data container
    message: DataDto
}

impl MessageDto {
    pub fn new(
        queue_id: i64,
        chat_id: i64,
        first_name: String,
        last_name: Option<String>,
        username: String,
        chat_type: String,
        date: i64,
        text: String
    ) -> Self {
        return MessageDto { 
            update_id: queue_id,
            message: DataDto { 
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

impl contract::response::MessageDtoInterface for MessageDto {
    fn get_queue_id(&self) -> i64 {
        return self.update_id;
    }

    fn get_chat_id(&self) -> i64 {
        return self.message.chat.id;
    }
    
    fn get_first_name(&self) -> &str {
        return &self.message.chat.first_name;
    }
    
    fn get_last_name(&self) -> Option<&str> {
        if let Some(last_name) = &self.message.chat.last_name {
            return Some(last_name);
        }
        return None;
    }
    
    fn get_username(&self) -> &str {
        return &self.message.chat.username;
    }
    
    fn get_chat_type(&self) -> &str {
        return &self.message.chat.r#type;
    }

    fn get_date(&self) -> i64 {
        return self.message.date;
    }
    
    fn get_text(&self) -> &str {
        return &self.message.text;
    }
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
    // user id
    id: i64,
    // user first name 
    first_name: String,
    // user last name
    last_name: Option<String>,
    // nickname of user in telegram
    username: String,
    // user chat type: channel, private chat, public chat
    r#type: String,
}

impl contract::response::ChatDtoInterface for ChatDto {
    fn get_chat_id(&self) -> i64 {
        return self.id;
    }

    fn get_first_name(&self) -> &str {
        return &self.first_name;
    }

    fn get_last_name(&self) -> Option<&str> {
        if let Some(last_name) = &self.last_name {
            return Some(last_name)
        }
        return None
    }

    fn get_username(&self) -> &str {
        return &self.username;
    }
    
    fn get_type(&self) -> &str {
        return &self.r#type;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FromDto {
    // user id
    id: i64,
    // is bot - indicator
    is_bot: bool,
    // user first name 
    first_name: String,
    // nickname of user in telegram
    username: String,
    // user last name
    last_name: Option<String>,
    // user language
    language_code: Option<String>,
    // is premium user - indicator
    is_premium: Option<bool>
}

impl contract::response::FromDtoInterface for FromDto {
    fn get_user_id(&self) -> i64 {
        return self.id;
    }

    fn get_first_name(&self) -> &str {
        return &self.first_name;
    }

    fn get_last_name(&self) -> Option<&str> {
        if let Some(last_name) = &self.last_name {
            return Some(last_name);
        }
        return None;
    }

    fn get_username(&self) -> &str {
        return &self.username;
    }

    fn get_language_code(&self) -> Option<&str> {
        if let Some(language_code) = &self.language_code {
            return Some(language_code);
        }
        return None;
    }

    fn is_bot(&self) -> bool {
        return self.is_bot;
    }

    fn is_premium(&self) -> Option<bool> {
        if let Some(is_premium) = self.is_premium {
            return Some(is_premium);
        }
        return None;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessSendMessageDto {
    // is ok? - request status
    ok: bool,
    // message struct which has data on success request
    result: ResultDto
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FailedSendMessageDto {
    // is ok? - request status
    ok: bool,
    // code of error on failed request
    pub error_code: i32,
    // description of error on failed request
    pub description: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultDto {
    // external message primary key
    message_id: i64,
    // whom sent a message
    from: FromDto,
    // which chat was used
    chat: ChatDto,
    // unix timestamp
    date: i64,
    // message text
    text: String
}

impl contract::response::SendMessageDtoInterface for SuccessSendMessageDto {
    fn is_ok(&self) -> bool {
        return self.ok;
    }

    fn get_message_id(&self) -> i64 {
        return self.result.message_id;
    }

    fn get_from_data(&self) -> &FromDto {
        return &self.result.from;
    }

    fn get_chat_data(&self) -> &ChatDto {
        return &self.result.chat;
    }

    fn get_date(&self) -> i64 {
        return self.result.date;
    }

    fn get_text(&self) -> &str {
        return &self.result.text;
    }
}