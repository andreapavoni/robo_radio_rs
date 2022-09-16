use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

use super::{
    api_data::{PlaylistResponse, TrackResponse, TrackStreamResponse},
    Playlist, Track,
};
use crate::errors::Error;

pub async fn get_playlist_tracks(
    client_id: String,
    playlist_id: String,
) -> Result<Playlist, Error> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        // Trace HTTP requests. See the tracing crate to make use of these traces.
        // Retry failed requests.
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let res = client
        .get(format!(
            "https://api-v2.soundcloud.com/playlists/{}?client_id={}",
            playlist_id, client_id
        ))
        .send()
        .await
        .map_err(|e| Error::SoundcloudRequestError(e))?;

    if !res.status().is_success() {
        return Err(Error::SoundcloudResponseError(res.status().as_u16()));
    }

    match res.json::<PlaylistResponse>().await {
        Ok(res) => Ok(res.into()),
        Err(_) => Err(Error::SoundcloudJsonParseError(String::from(
            "PlaylistResponse",
        ))),
    }
}

pub async fn get_track(client_id: String, track_id: u64) -> Result<Track, Error> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        // Retry failed requests.
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let res = client
        .get(format!(
            "https://api-v2.soundcloud.com/tracks/{}?client_id={}",
            track_id, client_id
        ))
        .send()
        .await
        .map_err(|e| Error::SoundcloudRequestError(e))?;

    if !res.status().is_success() {
        return Err(Error::SoundcloudResponseError(res.status().as_u16()));
    }

    match res.json::<TrackResponse>().await {
        Ok(res) => Ok(res.into()),
        Err(_) => Err(Error::SoundcloudJsonParseError(String::from(
            "TrackResponse",
        ))),
    }
}

pub async fn get_track_stream(
    client_id: String,
    track_url: String,
) -> Result<TrackStreamResponse, Error> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        // Retry failed requests.
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let url = format!("{}?client_id={}", track_url, client_id);

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| Error::SoundcloudRequestError(e))?;

    if !res.status().is_success() {
        return Err(Error::SoundcloudResponseError(res.status().as_u16()));
    }

    match res.json::<TrackStreamResponse>().await {
        Ok(res) => Ok(res),
        Err(_) => Err(Error::SoundcloudJsonParseError(String::from(
            "TrackResponse",
        ))),
    }
}
