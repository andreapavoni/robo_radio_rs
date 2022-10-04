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
pub struct CurrentTrack {
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

impl CurrentTrack {
    pub fn new(track: &Track) -> Self {
        Self {
            started_at: Utc::now(),
            id: track.id,
            permalink_url: track.permalink_url.as_ref().unwrap().clone(),
            // artwork_url: track.artwork_url.as_ref().unwrap().clone(),
            duration: *track.duration.as_ref().unwrap(),
            title: track.title.as_ref().unwrap().clone(),
            artist: track.artist.as_ref().unwrap().clone(),
            artist_permalink: track.artist_permalink.as_ref().unwrap().clone(),
            url: track.url.as_ref().unwrap().clone(),
            token: track.token.as_ref().unwrap().clone(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct MediaPlayer {
    playlist_id: Option<String>,
    client_id: String,
    client_id_timestamp: DateTime<Utc>,
    api: ApiClient,
    tracks_ids: Vec<u64>,
    pub current_track: Option<CurrentTrack>,
}

impl MediaPlayer {
    pub async fn new() -> Result<Self, Error> {
        let api = ApiClient::new();
        let client_id = api.get_client_id().await?;
        let client_id_timestamp = Utc::now();

        Ok(Self {
            api,
            client_id,
            client_id_timestamp,
            tracks_ids: vec![],
            current_track: None,
            playlist_id: None,
        })
    }

    pub async fn refresh_client_id(&mut self) -> Result<(), Error> {
        let client_id = self.api.get_client_id().await?;
        self.client_id = client_id;
        self.client_id_timestamp = Utc::now();
        Ok(())
    }

    pub async fn load_playlist(&mut self, playlist_id: &str) -> Result<(), Error> {
        let mut playlist = self
            .api
            .get_playlist(self.client_id.as_ref(), playlist_id.as_ref())
            .await?;

        self.playlist_id = Some(playlist_id.to_string());
        playlist.tracks_ids.shuffle(&mut thread_rng());
        self.tracks_ids = playlist.tracks_ids.clone();

        tracing::info!(
            "(re)loaded playlist with {} tracks",
            playlist.tracks_ids.clone().len()
        );

        Ok(())
    }

    pub async fn load_next_track(&mut self) -> Result<(), Error> {
        loop {
            self.ensure_client_id_validity().await?;
            self.ensure_playlist_not_empty().await?;

            let track_id = self.tracks_ids.pop().unwrap();
            if let Ok(track) = self.api.get_track(self.client_id.as_ref(), track_id).await {
                self.current_track = Some(CurrentTrack::new(&track));
                break;
            }
            tracing::warn!("skipping track with id {} because of some error", track_id);
            continue;
        }
        Ok(())
    }

    async fn ensure_client_id_validity(&mut self) -> Result<(), Error> {
        let now = Utc::now();
        let elapsed = now.signed_duration_since(self.client_id_timestamp);
        if elapsed.num_days() >= 1 {
            self.refresh_client_id().await?;
        }
        Ok(())
    }

    async fn ensure_playlist_not_empty(&mut self) -> Result<(), Error> {
        if self.tracks_ids.is_empty() {
            self.load_playlist(self.clone().playlist_id.as_ref().unwrap().as_str())
                .await?;
        }
        Ok(())
    }
}
