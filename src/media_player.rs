use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;

use crate::{
    error::Error,
    soundcloud::{ApiClient, Track},
};

#[derive(Debug, Clone, Serialize)]
pub struct Playback {
    pub started_at: DateTime<Utc>,
    pub id: u64,
    pub permalink_url: String,
    // pub artwork_url: String,
    pub duration: u64,
    pub title: String,
    pub artist: String,
    pub artist_permalink: String,
    pub url: String,
    pub token: String,
}

impl Playback {
    pub fn new(track: Track) -> Self {
        Self {
            started_at: Utc::now(),
            id: track.id,
            permalink_url: track.permalink_url.unwrap(),
            // artwork_url: track.artwork_url.unwrap(),
            duration: track.duration.unwrap(),
            title: track.title.unwrap(),
            artist: track.artist.unwrap(),
            artist_permalink: track.artist_permalink.unwrap(),
            url: track.url.unwrap(),
            token: track.token.unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MediaPlayer {
    playlist_id: Option<String>,
    client_id: String,
    api: ApiClient,
    tracks_ids: Vec<u64>,
    pub playback: Option<Playback>,
}

impl MediaPlayer {
    pub fn new(client_id: String) -> Self {
        Self {
            client_id,
            tracks_ids: vec![],
            playback: None,
            api: ApiClient::new(),
            playlist_id: None,
        }
    }

    pub async fn load_playlist(&mut self, playlist_id: String) -> Result<(), Error> {
        let mut playlist = self
            .api
            .get_playlist(self.client_id.clone(), playlist_id.clone())
            .await?;

        self.playlist_id = Some(playlist_id);
        playlist.tracks_ids.shuffle(&mut thread_rng());
        self.tracks_ids = playlist.tracks_ids;

        Ok(())
    }

    pub async fn load_next_track(&mut self) -> Result<(), Error> {
        if let Some(track_id) = self.tracks_ids.pop() {
            let track = self.api.get_track(self.client_id.clone(), track_id).await?;
            self.playback = Some(Playback::new(track));
        } else {
            self.load_playlist(self.playlist_id.as_ref().unwrap().to_string())
                .await?;
        }

        Ok(())
    }
}
