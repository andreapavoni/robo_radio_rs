use reqwest::Error as ReqwestError;
use reqwest_middleware::Error as MiddlewareReqwestError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error parsing JSON")]
    SoundcloudJsonParseError(String),
    #[error(transparent)]
    SoundcloudRequestError(#[from] MiddlewareReqwestError),
    #[error("error from SoundCloud response with code `{0}`")]
    SoundcloudResponseError(u16),
    #[error("error from SoundCloud text response")]
    SoundcloudTextResponseError(#[from] ReqwestError),
    #[error("track from SoundCloud with title `{0}` is incomplete")]
    SoundcloudIncompleteTrack(String),
    #[error(transparent)]
    WebSocketError(#[from] axum::Error),
}
