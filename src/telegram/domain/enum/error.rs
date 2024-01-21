#[derive(Debug)]
enum Error {
    HttpRequestError(reqwest::Error),
    TelegramError(String)
}