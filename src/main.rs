mod domain;

#[macro_use]
extern crate serde_derive;
extern crate serde_traitobject;
extern crate serde_json;

use serde_traitobject::Box;

use crate::domain::telegram::gateway::contract;
use crate::domain::telegram::gateway::contract::response::GetMessagesDtoInterface;
use crate::domain::telegram::gateway::dto;


fn main() {
    let get_messages_request_dto = dto::request::GetMessagesDto::new(1234567);
    let send_message_request_dto = dto::request::SendMessageDto::new(5678, "Hello from struct".to_string(), "html".to_string());

    println!("req: '{:?}', resp: '{:?}'\n\n", get_messages_request_dto, send_message_request_dto);

    let message_response_dto1 = dto::response::MessageDto::new(
        123, 
        234, 
        "Jared".to_string(),
        "Jaredson".to_string(),
        "Jareddd".to_string(),
        "channel".to_string(),
        32843599832470,
        "message from channel Jared".to_string()
    );
    let message_response_dto2 = dto::response::MessageDto::new(
        1235, 
        2347, 
        "Jack".to_string(),
        "Jackson".to_string(),
        "Jackkkk".to_string(),
        "channel".to_string(),
        32843599345470,
        "message from channel of Jack".to_string()
    );

    let messages_response_dto: Vec<Box<dyn contract::response::MessageDtoInterface>> = vec![Box::new(message_response_dto1), Box::new(message_response_dto2)];
    let get_messages_response_dto = dto::response::GetMessagesDto::new(messages_response_dto);

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






// #[macro_use]
// extern crate serde_derive;

// #[macro_use]
// extern crate erased_serde;

// extern crate serde;
// extern crate serde_json;

// #[derive(Serialize)]
// struct Card {
//     sections: Vec<Section>,
// }

// #[derive(Serialize)]
// struct Section {
//     header: String,
//     widgets: Vec<Box<dyn WidgetTrait>>,
// }

// #[derive(Serialize)]
// struct Image {
//     image_url: String,
// }

// #[derive(Serialize)]
// struct KeyValue {
//     top_label: String,
//     content: String,
// }

// trait WidgetTrait: erased_serde::Serialize {}
// impl WidgetTrait for Image {}
// impl WidgetTrait for KeyValue {}

// serialize_trait_object!(WidgetTrait);

// fn main() {
//     let card = Card {
//         sections: vec![
//             Section {
//                 header: "text".to_owned(),
//                 widgets: vec![
//                     Box::new(Image {
//                         image_url: "img".to_owned(),
//                     }),
//                     Box::new(KeyValue {
//                         top_label: "text".to_owned(),
//                         content: "text".to_owned(),
//                     }),
//                 ],
//             },
//         ],
//     };

//     println!("{}", serde_json::to_string_pretty(&card).unwrap());
// }






// #[macro_use]
// extern crate serde_derive;
// extern crate serde_traitobject;
// extern crate serde_json;
// use std::any::Any;
// use serde_traitobject::{self as s, Debug};

// fn main() {
//     trait BazInterface: Debug  {

//     }

//     #[derive(Serialize, Deserialize, Debug)]
//     struct BazStruct {
//         any: String
//     }

//     impl BazInterface for BazBazStruct {

//     }

//     #[derive(Serialize, Deserialize, Debug)]
//     struct BazBazStruct {
//         any1: String
//     }

//     impl BazInterface for BazStruct {

//     }

//     #[derive(Serialize, Deserialize, Debug)]
//     struct MyStruct {
//         foo: String,
//         bar: usize,
//         baz: Vec<s::Box<dyn BazInterface>>
//     }

//     let my_struct = MyStruct {
//         foo: String::from("abc"),
//         bar: 123,
//         baz: vec![
//             s::Box::new(BazStruct{any: "hello world".to_string()}),
//             s::Box::new(BazBazStruct{any1: "hello world agein".to_string()})
//         ]
//     };

//     let erased: s::Box<dyn s::Any> = s::Box::new(my_struct);

//     let serialized = serde_json::to_string(&erased).unwrap();

//     let deserialized: s::Box<dyn s::Any> = serde_json::from_str(&serialized).unwrap();

//     let downcast: Box<MyStruct> = Box::<dyn Any>::downcast(deserialized.into_any()).unwrap();

//     println!(
//         "
//         serialized = {}, \n
//         downcast = {:?}\n\n
//         ",
//         serialized,
//         downcast
//     );
// }