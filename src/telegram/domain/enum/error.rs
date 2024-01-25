#[derive(Debug)]
pub enum Error {
    InternalError(Box<dyn std::error::Error>),
    TelegramError(Box<dyn std::error::Error>),
    RepositoryError(Box<dyn std::error::Error>),
}