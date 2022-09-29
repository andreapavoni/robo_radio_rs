use reqwest_middleware::Error as MiddlewareReqwestError;

#[derive(Debug)]
pub enum Error {
    SoundcloudJsonParseError(String),
    SoundcloudRequestError(MiddlewareReqwestError),
    SoundcloudResponseError(u16),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Error::SoundcloudJsonParseError(err) => {
                write!(f, "Soundcloud JSON parse error: {}", err)
            }
            Error::SoundcloudRequestError(err) => {
                write!(f, "Soundcloud API request error: {}", err)
            }
            Error::SoundcloudResponseError(err) => {
                write!(f, "Soundcloud API error: status {}", err)
            }
        }
    }
}
