mod domain;

#[macro_use]
extern crate serde_derive;
extern crate serde_traitobject;
extern crate serde_json;

use core::panic;

use serde_traitobject::Box;

use crate::domain::telegram::gateway::contract;
use crate::domain::telegram::gateway::contract::gateway::GatewayInterface;
use crate::domain::telegram::gateway::dto;
use crate::domain::telegram::gateway::usecase;


fn main() {
    let gateway = usecase::gateway::Gateway::new(
        "https://api.telegram.org".to_string(),
        "6283148707:AAE34Fob6V8cDlgzhspHKv7TX-r2CLdYXTs".to_string()
    );

    // request: get messages
    let request_dto: Box<dyn contract::request::GetMessagesDtoInterface> =
        Box::new(dto::request::GetMessagesDto::new(904019216));

    let result = gateway.get_messages(request_dto);

    let response = match result {
        Ok(v) => v,
        Err(err) => panic!("{}", err)
    };

    println!("RESPONSE OF GET MESSAGES: {:?}", response);

    // request: send message
    let send_message_request_dto: Box<dyn contract::request::SendMessageDtoInterface> =
        Box::new(dto::request::SendMessageDto::new(1063099947, "Hello from code".to_string(), "html".to_string()));

    let result_of_send_message_request = gateway.send_message(send_message_request_dto);

    let response_of_send_message_request = match result_of_send_message_request {
        Ok(v) => v,
        Err(err) => panic!("{}", err)
    };

    println!("RESPONSE OF SEND MESSAGE: {:?}", response_of_send_message_request)
}




// use serde::Serialize;
// use serde_traitobject::Box;
// use std::fmt::Debug;
// fn main() {
//     trait IncMessageInterface: Debug + serde_traitobject::Serialize {}
//
//     #[derive(Debug, Serialize)]
//     struct IncMessage {}
//
//     impl IncMessageInterface for IncMessage {}
//
//
//     trait MessageInterface: serde_traitobject::Serialize {
//         fn get_data(&self) -> String;
//         fn get_inc_msgs(&self) -> Vec<Box<dyn IncMessageInterface>>;
//     }
//
//     #[derive(Debug, Serialize)]
//     struct Message {
//         msg: String,
//         inc_msgs: Vec<Box<IncMessage>>
//     }
//
//     impl MessageInterface for Message {
//         fn get_data(&self) -> String {
//             return self.msg.clone();
//         }
//
//         fn get_inc_msgs(&self) -> Vec<Box<dyn IncMessageInterface>> {
//             let mut inc_msgs: Vec<Box<dyn IncMessageInterface>> = vec![];
//
//             for _inc_msg in &self.inc_msgs {
//                 inc_msgs.push(Box::new(IncMessage{}));
//             }
//
//             return inc_msgs;
//         }
//     }
//
//     let message: Vec<Box<dyn MessageInterface>> = vec![
//         Box::new(
//             Message {
//                 msg: "strstrstr".to_string(),
//                 inc_msgs: vec![Box::new(IncMessage{}), Box::new(IncMessage{}), Box::new(IncMessage{})]
//             }
//         )
//     ];
//
//     let msg = message.first().unwrap();
//
//     let json = serde_json::to_string(msg).unwrap();
//
//     println!("{}", json);
//
//     // println!("{}, {:?}", msg.get_data(), msg.get_inc_msgs());
// }




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
//         fn get_msg(&self) -> String;
//     }

//     #[derive(Serialize, Deserialize, Debug)]
//     struct BazStruct {
//         any: String,
//         msg: String
//     }

//     impl BazInterface for BazBazStruct {
//         fn get_msg(&self) -> String {
//             return self.msg.clone();
//         }
//     }

//     #[derive(Serialize, Deserialize, Debug)]
//     struct BazBazStruct {
//         any1: String,
//         msg: String
//     }

//     impl BazInterface for BazStruct {
//         fn get_msg(&self) -> String {
//             return self.msg.clone();
//         }
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
//             s::Box::new(BazStruct{any: "hello world".to_string(), msg: "BazStruct implementation".to_string()}),
//             s::Box::new(BazBazStruct{any1: "hello world agein".to_string(), msg: "BazBazStruct implementation".to_string()})
//         ]
//     };

//     let erased: s::Box<dyn s::Any> = s::Box::new(my_struct);

//     // let serialized = serde_json::to_string(&erased).unwrap();
//     let serialized: String = "
//     [{
//         \"foo\": \"abc\",
//         \"bar\": 123,
//         \"baz\": [

//                 [{
//                     \"any\": \"hello world\",
//                     \"msg\": \"BazStruct implementation\"
//                 }]
//             ,

//                 [{
//                     \"any1\": \"hello world agein\",
//                     \"msg\": \"BazBazStruct implementation\"
//                 }]

//         ]
//     }]
//     ".to_string();

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
