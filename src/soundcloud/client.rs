use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

use super::{
    api_data::{Playlist as PlaylistResponse, Track as TrackResponse},
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
        Ok(res) => Ok(transform_playlist_response(res).await),
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
        Ok(res) => Ok(transform_track_response(res).await),
        Err(_) => Err(Error::SoundcloudJsonParseError(String::from(
            "TrackResponse",
        ))),
    }
}

async fn transform_playlist_response(res: PlaylistResponse) -> Playlist {
    Playlist {
        tracks_ids: res.tracks.into_iter().map(|t| t.id).collect(),
    }
}

async fn transform_track_response(res: TrackResponse) -> Track {
    let track_url = res
        .media
        .unwrap()
        .transcodings
        .into_iter()
        .find(|ts| ts.format.protocol == String::from("progressive"))
        .unwrap()
        .url;

    let user = res.user.unwrap();

    Track {
        id: res.id,
        permalink_url: res.permalink_url,
        artwork_url: res.artwork_url,
        duration: res.duration,
        title: res.title,
        artist: Some(user.username),
        artist_permalink: Some(user.permalink_url),
        url: Some(track_url),
        token: res.track_authorization,
    }
}
