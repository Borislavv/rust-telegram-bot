use crate::domain::telegram::gateway::r#enum::Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::HttpRequestError(ref err) => write!(f, "HTTP request error: {}", err),
            Error::TelegramError(ref err) => write!(f, "Telegram gateway error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::HttpRequestError(ref err) => Some(err),
            Error::TelegramError(_) => None,
        }
    }
}