use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{
    error::Error,
    soundcloud::{ApiClient, Track},
};

#[derive(Debug, Clone)]
pub struct MediaPlayer {
    playlist_id: Option<String>,
    client_id: String,
    api: ApiClient,
    tracks_ids: Vec<u64>,
    pub current_track: Option<Track>,
}

impl MediaPlayer {
    pub fn new(client_id: String, api: ApiClient) -> Self {
        Self {
            client_id,
            tracks_ids: vec![],
            current_track: None,
            api,
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
            self.current_track = Some(track);
        } else {
            self.load_playlist(self.playlist_id.as_ref().unwrap().to_string())
                .await?;
        }

        Ok(())
    }
}
