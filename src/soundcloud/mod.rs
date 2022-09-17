use crate::errors::Error;
use std::convert::From;

use self::client::{
    fetch_playlist_tracks, fetch_track_info, fetch_track_stream, PlaylistResponse, TrackResponse,
};

mod client;

#[derive(Debug)]
pub struct ApiClient {
    pub client_id: String,
}

impl ApiClient {
    pub fn new(client_id: String) -> Self {
        ApiClient { client_id }
    }

    pub async fn get_track(&self, track_id: u64) -> Result<Track, Error> {
        let mut track = fetch_track_info(self.client_id.clone(), track_id).await?;
        let track_stream = fetch_track_stream(
            self.client_id.clone(),
            track.url.unwrap(),
            track.token.as_ref().unwrap().to_string(),
        )
        .await?;
        track.url = track_stream.url.clone();
        Ok(track.clone())
    }

    pub async fn get_playlist(&self, playlist_id: String) -> Result<Playlist, Error> {
        fetch_playlist_tracks(self.client_id.clone(), playlist_id).await
    }
}

#[derive(Debug)]
pub struct Playlist {
    pub tracks_ids: Vec<u64>,
}

impl std::fmt::Display for Playlist {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.tracks_ids)
    }
}

impl From<PlaylistResponse> for Playlist {
    fn from(playlist: PlaylistResponse) -> Self {
        Playlist {
            tracks_ids: playlist.tracks.into_iter().map(|t| t.id).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: u64,
    pub permalink_url: Option<String>,
    pub artwork_url: Option<String>,
    pub duration: Option<u64>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub artist_permalink: Option<String>,
    pub url: Option<String>,
    pub token: Option<String>,
}

impl From<TrackResponse> for Track {
    fn from(track: TrackResponse) -> Self {
        let track_url = track
            .media
            .unwrap()
            .transcodings
            .into_iter()
            .find(|ts| ts.format.protocol == String::from("progressive"))
            .unwrap()
            .url;

        let user = track.user.unwrap();

        Track {
            id: track.id,
            permalink_url: track.permalink_url,
            artwork_url: track.artwork_url,
            duration: track.duration,
            title: track.title,
            artist: Some(user.username),
            artist_permalink: Some(user.permalink_url),
            url: Some(track_url),
            token: track.track_authorization,
        }
    }
}
