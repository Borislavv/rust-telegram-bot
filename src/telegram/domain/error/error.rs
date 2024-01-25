use crate::telegram::domain::r#enum;
use std::fmt;

impl fmt::Display for r#enum::error::Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            r#enum::error::Error::InternalError(ref err) => write!(f, "Internal error occurred: {}", err),
            r#enum::error::Error::TelegramError(ref err) => write!(f, "Telegram API error occurred: {}", err),
            r#enum::error::Error::RepositoryError(ref err) => write!(f, "Http repository error occurred: {}", err),
        }
    }
}

impl std::error::Error for r#enum::error::Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            r#enum::error::Error::InternalError(ref err) => Some(err.as_ref()),
            r#enum::error::Error::TelegramError(ref err) => Some(err.as_ref()),
            r#enum::error::Error::RepositoryError(ref err) => Some(err.as_ref()),
        }
    }
}