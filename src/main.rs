use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_extra::routing::SpaRouter;
use futures::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
// use std::{env, sync::Arc};
use futures::FutureExt;
use std::{collections::HashMap, env, sync::Arc};
use tokio::{
    sync::{
        mpsc::{unbounded_channel, UnboundedSender},
        Mutex,
    },
    time::{sleep, Duration},
};

use robo_radio_rs::{error::Error, media_player::MediaPlayer, soundcloud::ApiClient};

// Shared state
#[derive(Debug, Clone)]
struct AppState {
    media_player: MediaPlayer,
    clients: Clients,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<UnboundedSender<Result<Message, axum::Error>>>,
}

type Clients = HashMap<String, Client>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client_id = match env::var_os("ROBO_RADIO_SOUNDCLOUD_CLIENT_ID") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$ROBO_RADIO_SOUNDCLOUD_CLIENT_ID is not set"),
    };

    let playlist_id = match env::var_os("ROBO_RADIO_SOUNDCLOUD_PLAYLIST_ID") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$ROBO_RADIO_SOUNDCLOUD_PLAYLIST_ID is not set"),
    };

    let api = ApiClient::new();
    let mut media_player = MediaPlayer::new(client_id, api);
    let clients: Clients = HashMap::new();

    media_player.load_playlist(playlist_id).await?;
    media_player.load_next_track().await?;

    let state = Arc::new(Mutex::new(AppState {
        clients,
        media_player,
    }));

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/socket/websocket", get(websocket_handler))
        .merge(SpaRouter::new("/assets", "assets"))
        .layer(Extension(state.clone()));

    tokio::spawn(async move {
        loop {
            let current_track = state
                .lock()
                .await
                .media_player
                .playback
                .as_ref()
                .unwrap()
                .clone();
            println!("current_track inside loop {:?}", current_track.title);

            broadcast_msg(
                Message::Text(serde_json::json!({ "payload": current_track }).to_string()),
                &state,
            )
            .await;

            let duration = Duration::from_millis(current_track.duration);
            sleep(duration).await;
            let _ = state.lock().await.media_player.load_next_track().await;
        }
    });

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// Include utf-8 file at **compile** time.
async fn index_handler() -> Html<&'static str> {
    Html(std::include_str!("../frontend/index.html"))
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

async fn websocket_connection(ws: WebSocket, state: Arc<Mutex<AppState>>) {
    println!("establishing client connection...");

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));

    let uuid = Uuid::new_v4().as_simple().to_string();

    println!("client connected with id: {}", uuid.clone());

    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender.clone()),
    };

    state.lock().await.clients.insert(uuid.clone(), new_client);

    let current_track = state
        .lock()
        .await
        .media_player
        .playback
        .as_ref()
        .unwrap()
        .clone();
    println!("current_track inside loop {:?}", current_track.title);

    let notification = Message::Text(serde_json::json!({ "payload": current_track }).to_string());
    let _ = client_sender.send(Ok(notification));

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };
        handle_client_msg(&uuid, msg, &state).await;
    }

    state.lock().await.clients.remove(&uuid);
    println!("{} disconnected", uuid);
}

async fn handle_client_msg(client_id: &str, msg: Message, state: &Arc<Mutex<AppState>>) {
    println!("received message from {}: {:?}", client_id, msg);

    let message = match msg.to_text() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message == "ping\n" {
        let clients = state.lock().await.clients.clone();
        match clients.get(client_id) {
            Some(v) => {
                if let Some(sender) = &v.sender {
                    println!("sending pong");
                    let _ = sender.send(Ok(Message::Text(String::from("pong"))));
                }
            }
            None => return,
        }
        return;
    };
}

async fn broadcast_msg(msg: Message, state: &Arc<Mutex<AppState>>) {
    // println!("broadcast message {:?}", msg);

    let clients = state.lock().await.clients.clone();

    for (_id, client) in clients.into_iter() {
        if let Some(sender) = &client.sender {
            println!("sending broadcast message");
            let _ = sender.send(Ok(msg.clone()));
        }
    }
    return;
}
