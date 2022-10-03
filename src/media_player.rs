use crate::{
    error::Error,
    soundcloud::{ApiClient, Track},
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;

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
    pub fn new(track: &Track) -> Self {
        Self {
            started_at: Utc::now(),
            id: track.id,
            permalink_url: track.permalink_url.as_ref().unwrap().clone(),
            // artwork_url: track.artwork_url.as_ref().unwrap().clone(),
            duration: track.duration.as_ref().unwrap().clone(),
            title: track.title.as_ref().unwrap().clone(),
            artist: track.artist.as_ref().unwrap().clone(),
            artist_permalink: track.artist_permalink.as_ref().unwrap().clone(),
            url: track.url.as_ref().unwrap().clone(),
            token: track.token.as_ref().unwrap().clone(),
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
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            tracks_ids: vec![],
            playback: None,
            api: ApiClient::new(),
            playlist_id: None,
        }
    }

    pub async fn load_playlist(&mut self, playlist_id: &str) -> Result<(), Error> {
        let mut playlist = self
            .api
            .get_playlist(self.client_id.as_ref(), playlist_id.as_ref())
            .await?;

        self.playlist_id = Some(playlist_id.to_string());
        playlist.tracks_ids.shuffle(&mut thread_rng());
        self.tracks_ids = playlist.tracks_ids.clone();

        Ok(())
    }

    pub async fn load_next_track(&mut self) -> Result<(), Error> {
        if self.tracks_ids.len() < 1 {
            self.load_playlist(self.clone().playlist_id.as_ref().unwrap().as_str())
                .await?;
        }

        let track_id = self.tracks_ids.pop().unwrap();
        let track = self
            .api
            .get_track(self.client_id.as_ref(), track_id)
            .await?;
        self.playback = Some(Playback::new(&track));
        Ok(())
    }
}
