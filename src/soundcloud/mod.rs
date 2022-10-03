use self::client::{
    fetch_playlist_tracks, fetch_track_info, fetch_track_stream, PlaylistResponse, TrackResponse,
};
use crate::error::Error;
use anyhow::Result;
use serde::Serialize;
use std::convert::From;

mod client;

#[derive(Debug, Clone)]
pub struct ApiClient {}

impl ApiClient {
    pub fn new() -> Self {
        ApiClient {}
    }

    pub async fn get_track(&self, client_id: &str, track_id: u64) -> Result<Track, Error> {
        let mut track = fetch_track_info(client_id, track_id).await?;
        let track_stream = fetch_track_stream(
            client_id,
            track.url.unwrap().as_ref(),
            track.token.as_ref().unwrap(),
        )
        .await?;
        track.url = track_stream.url;
        Ok(track.clone())
    }

    pub async fn get_playlist(
        &self,
        client_id: &str,
        playlist_id: &str,
    ) -> Result<Playlist, Error> {
        fetch_playlist_tracks(client_id, playlist_id).await
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
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

#[derive(Debug, Clone, Serialize)]
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

impl TryFrom<TrackResponse> for Track {
    type Error = Error;

    fn try_from(track: TrackResponse) -> Result<Self, Error> {
        let track_url = match track
            .media
            .unwrap()
            .transcodings
            .into_iter()
            .find(|ts| ts.format.protocol == *"progressive")
        {
            Some(transcoding) => Some(transcoding.url),
            _ => return Err(Error::SoundcloudIncompleteTrack(track.title.unwrap())),
        };

        let user = track.user.unwrap();

        Ok(Track {
            id: track.id,
            permalink_url: track.permalink_url,
            artwork_url: track.artwork_url,
            duration: track.duration,
            title: track.title,
            artist: Some(user.username),
            artist_permalink: Some(user.permalink_url),
            url: track_url,
            token: track.track_authorization,
        })
    }
}
