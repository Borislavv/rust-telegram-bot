use crate::telegram::domain::r#enum;
use std::fmt;
use std::str::Utf8Error;
use reqwest::Error;

impl fmt::Display for r#enum::error::Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            r#enum::error::Error::InternalError(ref err) => write!(f, "Internal error occurred: {}", err),
            r#enum::error::Error::TelegramError(ref err) => write!(f, "Telegram API error occurred: {}", err),
            r#enum::error::Error::RepositoryError(ref err) => write!(f, "Http repository error occurred: {}", err),
            r#enum::error::Error::EncodeDecodeUTF8Error(ref err) => write!(f, "Encode/Decode UTF8 error occurred: {}", err),
            r#enum::error::Error::SerializeDeserializeError(ref err) => write!(f, "Serialize/Deserialize error occurred: {}", err),
        }
    }
}

impl From<String> for r#enum::error::Error {
    fn from(value: String) -> Self {
        r#enum::error::Error::InternalError(value)
    }
}

impl From<reqwest::Error> for r#enum::error::Error {
    fn from(value: Error) -> Self {
        r#enum::error::Error::RepositoryError(value.to_string())
    }
}

impl From<serde_json::Error> for r#enum::error::Error {
    fn from(value: serde_json::Error) -> Self {
        r#enum::error::Error::SerializeDeserializeError(value.to_string())
    }
}

impl From<Utf8Error> for r#enum::error::Error {
    fn from(value: Utf8Error) -> Self {
        r#enum::error::Error::EncodeDecodeUTF8Error(value.to_string())
    }
}