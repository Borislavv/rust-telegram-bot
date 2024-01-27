#[derive(Debug)]
pub enum Error {
    InternalError(String),
    TelegramError(String),
    RepositoryError(String),
    EncodeDecodeUTF8Error(String),
    SerializeDeserializeError(String)
}