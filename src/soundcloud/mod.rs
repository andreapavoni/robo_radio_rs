use api_data::{PlaylistResponse, TrackResponse};
use std::convert::From;

mod api_data;
pub mod client;

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

#[derive(Debug)]
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
