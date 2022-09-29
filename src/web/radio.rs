use super::ws::{Client, Clients, WebSocketHandler};
use crate::{
    error::Error,
    media_player::{MediaPlayer, Playback},
    web::ws::broadcast_message,
};
use async_trait::async_trait;
use axum::extract::ws::Message;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};

// Shared station
#[derive(Debug, Clone)]
pub struct Station {
    pub media_player: MediaPlayer,
    pub listeners: Clients,
}

impl Station {
    pub async fn new(client_id: String, playlist_id: String) -> Result<Station, Error> {
        let mut media_player = MediaPlayer::new(client_id);
        let listeners: Clients = HashMap::new();

        media_player.load_playlist(playlist_id).await?;
        media_player.load_next_track().await?;

        Ok(Station {
            listeners,
            media_player,
        })
    }

    // Utils
    pub async fn current_track(&mut self) -> Playback {
        self.media_player.playback.as_ref().unwrap().clone()
    }

    pub async fn next_track(&mut self) -> Result<(), Error> {
        self.media_player.load_next_track().await
    }
}

#[async_trait]
impl WebSocketHandler for Station {
    async fn on_connect(&mut self, client: &Client) {
        self.listeners.insert(client.clone().id, client.clone());

        let current_track = self.current_track().await;
        tracing::debug!(
            "notify client on current track started at {:?}: {:?}",
            current_track.started_at,
            current_track.title
        );

        // Notify client with the current playing track
        let notification =
            Message::Text(serde_json::json!({ "payload": current_track }).to_string());
        let _ = client.clone().sender.send(Ok(notification));
    }

    async fn on_disconnect(&mut self, client: &Client) {
        self.listeners.remove(&client.id);
        tracing::info!("client disconnected: {}", client.id);
    }

    async fn on_message(&mut self, _client: &Client) {}
}

pub type StationService = Arc<Mutex<Station>>;

pub async fn go_live(service: StationService) {
    loop {
        let current = service.lock().await.current_track().await;
        tracing::info!(
            "starting new track at {:?}: {:?}",
            current.started_at,
            current.title
        );

        broadcast_message(
            Message::Text(serde_json::json!({ "payload": current }).to_string()),
            &service.lock().await.listeners,
        )
        .await;

        let duration = Duration::from_millis(current.duration);
        sleep(duration).await;
        let _ = service.lock().await.next_track().await;
    }
}
