use crate::errors::Error;
use api_data::{PlaylistResponse, TrackResponse};
use std::convert::From;

use self::client::{get_playlist_tracks, get_track, get_track_stream};

mod api_data;
mod client;

#[derive(Debug)]
pub struct Playlist {
    pub tracks_ids: Vec<u64>,
}

impl Playlist {
    pub async fn new(client_id: String, playlist_id: String) -> Result<Playlist, Error> {
        get_playlist_tracks(client_id, playlist_id).await
    }
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

impl Track {
    pub async fn new(client_id: String, track_id: u64) -> Result<Track, Error> {
        let mut track = get_track(client_id.clone(), track_id).await?;
        let track_stream = get_track_stream(client_id, track.url.unwrap()).await?;
        track.url = track_stream.url.clone();
        Ok(track.clone())
    }
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
