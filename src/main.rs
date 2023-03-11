mod domain;

use crate::domain::telegram::gateway::dto;

fn main() {
    let get_messages_request_dto = dto::request::GetMessagesRequestDto::new(1234567);
    let send_message_request_dto = dto::request::SendMessageRequestDto::new(5678, "Hello from struct".to_string(), "html".to_string());

    println!("req: '{:?}', resp: '{:?}'", get_messages_request_dto, send_message_request_dto);
}
