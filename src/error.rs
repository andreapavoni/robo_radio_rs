use reqwest_middleware::Error as MiddlewareReqwestError;

#[derive(Debug)]
pub enum Error {
    SoundcloudJsonParseError(String),
    SoundcloudRequestError(MiddlewareReqwestError),
    SoundcloudResponseError(u16),
    SoundcloudIncompleteTrack(String),
    WebSocketError(axum::Error),
    PubSubError(String),
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
            Error::WebSocketError(err) => {
                write!(f, "WebSocket error: {:?}", err)
            }
            Error::SoundcloudIncompleteTrack(track_title) => {
                write!(
                    f,
                    "Soundcloud API error: incomplete track {:?}",
                    track_title
                )
            }
            Error::PubSubError(err) => {
                write!(f, "PubSub error: {}", err)
            }
        }
    }
}
