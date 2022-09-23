use axum::extract::ws::Message;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};

use super::socket::{Client, Clients};
use crate::{
    error::Error,
    media_player::{MediaPlayer, Playback},
    web::socket::send_broadcast,
};

// Shared station
#[derive(Debug, Clone)]
pub struct Channel {
    pub media_player: MediaPlayer,
    pub listeners: Clients,
}

impl Channel {
    pub async fn add_listener(&mut self, client_id: String, client: Client) {
        self.listeners.insert(client_id, client);
    }

    pub async fn remove_listener(&mut self, client_id: String) {
        self.listeners.remove(&client_id);
    }

    pub async fn current_track(&mut self) -> Playback {
        self.media_player.playback.as_ref().unwrap().clone()
    }
}

pub type Station = Arc<Mutex<Channel>>;

pub async fn new_station(client_id: String, playlist_id: String) -> Result<Station, Error> {
    let mut media_player = MediaPlayer::new(client_id);
    let listeners: Clients = HashMap::new();

    media_player.load_playlist(playlist_id).await?;
    media_player.load_next_track().await?;

    Ok(Arc::new(Mutex::new(Channel {
        listeners,
        media_player,
    })))
}

pub async fn go_live(station: Station) {
    loop {
        let current = station.lock().await.current_track().await;
        println!("new track at {:?}: {:?}", current.started_at, current.title);

        send_broadcast(
            Message::Text(serde_json::json!({ "payload": current }).to_string()),
            &station.lock().await.listeners,
        )
        .await;

        let duration = Duration::from_millis(current.duration);
        sleep(duration).await;
        let _ = station.lock().await.media_player.load_next_track().await;
    }
}
