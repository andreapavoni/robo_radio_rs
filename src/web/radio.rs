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
    media_player: MediaPlayer,
    listeners: Clients,
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

    async fn notify_listeners_count(&mut self) {
        broadcast_message(
            Message::Text(
                serde_json::json!({"event": "listeners", "data": self.listeners.keys().count()})
                    .to_string(),
            ),
            &self.listeners,
        )
        .await;
    }

    async fn build_current_track_msg(&self) -> Message {
        Message::Text(
            serde_json::json!({"event": "track", "data": self.clone().current_track().await})
                .to_string(),
        )
    }
}

#[async_trait]
impl WebSocketHandler for Station {
    async fn on_connect(&mut self, client: &Client) {
        self.listeners.insert(client.clone().id, client.clone());

        // Notify clients with listeners count
        self.notify_listeners_count().await;

        // Notify client with the current playing track
        let _ = client
            .clone()
            .send_message(self.build_current_track_msg().await);
    }

    async fn on_disconnect(&mut self, client: &Client) {
        self.listeners.remove(&client.id);
        self.notify_listeners_count().await;
        tracing::info!("client disconnected: {}", client.id);
    }

    async fn on_message(&mut self, _client: &Client) {}
}

pub type StationService = Arc<Mutex<Station>>;

pub async fn go_live(service: StationService) {
    loop {
        let track = service.lock().await.current_track().await;
        tracing::info!(
            "starting new track at {:?}: {:?}",
            track.started_at,
            track.title
        );

        broadcast_message(
            service.lock().await.build_current_track_msg().await,
            &service.lock().await.listeners,
        )
        .await;

        let duration = Duration::from_millis(track.duration);
        sleep(duration).await;
        let _ = service.lock().await.next_track().await;
    }
}
