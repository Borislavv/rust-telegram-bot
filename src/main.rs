mod domain;

use crate::domain::telegram::gateway::dto::{self, response::{MessageResponseDtoInterface, GetMessagesResponseDtoInterface}};

fn main() {
    let get_messages_request_dto = dto::request::GetMessagesRequestDto::new(1234567);
    let send_message_request_dto = dto::request::SendMessageRequestDto::new(5678, "Hello from struct".to_string(), "html".to_string());

    println!("req: '{:?}', resp: '{:?}'\n\n", get_messages_request_dto, send_message_request_dto);

    let message_response_dto1 = dto::response::MessageResponseDto::new(
        123, 
        234, 
        "Jared".to_string(),
        "Jaredson".to_string(),
        "Jareddd".to_string(),
        "channel".to_string(),
        32843599832470,
        "message from channel Jared".to_string()
    );
    let message_response_dto2 = dto::response::MessageResponseDto::new(
        1235, 
        2347, 
        "Jack".to_string(),
        "Jackson".to_string(),
        "Jackkkk".to_string(),
        "channel".to_string(),
        32843599345470,
        "message from channel of Jack".to_string()
    );

    let messages_response_dto: Vec<Box<dyn MessageResponseDtoInterface>> = vec![Box::new(message_response_dto1), Box::new(message_response_dto2)];
    let get_messages_response_dto = dto::response::GetMessagesResponseDto::new(messages_response_dto);

    for msg_resp in get_messages_response_dto.get_messages() {
        println!(
            "
                queue_id = {},
                chat_id = {},
                firstname = {},
                lastname = {},
                username = {},
                chat_type = {},
                date = {},
                text = {}
            ",
            msg_resp.get_queue_id(),
            msg_resp.get_chat_id(),
            msg_resp.get_first_name(),
            msg_resp.get_last_name(),
            msg_resp.get_username(),
            msg_resp.get_chat_type(),
            msg_resp.get_date(),
            msg_resp.get_text()
        );
    }
}
