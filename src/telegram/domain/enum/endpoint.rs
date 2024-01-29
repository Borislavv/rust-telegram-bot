#[derive(Eq, Hash, PartialEq)]
pub enum Endpoint {
    GetMessagesMethod,
    SendMessageMethod,
}

impl Endpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Endpoint::GetMessagesMethod => "getUpdates",
            Endpoint::SendMessageMethod => "sendMessage",
        }
    }
}