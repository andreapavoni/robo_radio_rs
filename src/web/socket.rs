use std::collections::HashMap;

use axum::extract::ws::{Message, WebSocket};
use futures::StreamExt;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;

use futures::FutureExt;
use uuid::Uuid;

use super::radio::Station;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<UnboundedSender<Result<Message, axum::Error>>>,
}

pub type Clients = HashMap<String, Client>;

pub async fn websocket_connection(ws: WebSocket, station: Station) {
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

    station
        .lock()
        .await
        .add_listener(uuid.clone(), new_client)
        .await;

    let current_track = station.lock().await.current_track().await;
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
        handle_message_from_client(&uuid, msg, &station.lock().await.listeners).await;
    }

    station.lock().await.remove_listener(uuid.clone()).await;
    println!("{} disconnected", uuid);
}

async fn handle_message_from_client(client_id: &str, msg: Message, clients: &Clients) {
    println!("received message from {}: {:?}", client_id, msg);

    let message = match msg.to_text() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message == "ping\n" {
        if let Some(client) = clients.get(client_id) {
            println!("sending pong");
            send_client(Message::Text(String::from("pong")), client).await;
        }
    };
}

pub async fn send_broadcast(msg: Message, clients: &Clients) {
    for (_id, client) in clients.into_iter() {
        println!("sending broadcast message");
        send_client(msg.clone(), client).await;
    }
    return;
}

pub async fn send_client(msg: Message, client: &Client) {
    if let Some(sender) = &client.sender {
        let _ = sender.send(Ok(msg));
    }
}
