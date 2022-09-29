// use super::radio::Arc<Mutex<impl WebSocketHandler>>;
use async_trait::async_trait;
use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitStream;
use futures::FutureExt;
use futures::StreamExt;
use std::collections::HashMap;
use std::marker::Send;
use std::sync::Arc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::Mutex;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Client {
    pub id: String,
    pub sender: Sender,
}

impl Client {
    pub fn new(sender: Sender) -> Client {
        let id = Uuid::new_v4().as_simple().to_string();
        Client { id, sender }
    }

    pub async fn send_message(&self, msg: Message) {
        if self.sender.send(Ok(msg)).is_err() {
            tracing::error!("error sending message to client: {}", self.id)
        }
    }
}

pub type Sender = UnboundedSender<Result<Message, axum::Error>>;
pub type Clients = HashMap<String, Client>;
pub type WebSocketService = Arc<Mutex<dyn WebSocketHandler + Send>>;

#[async_trait]
pub trait WebSocketHandler {
    async fn on_connect(&mut self, client: &Client);
    async fn on_disconnect(&mut self, client: &Client);
    async fn on_message(&mut self, _client: &Client);
}

pub async fn handle_client_connection(ws: WebSocket, service: WebSocketService) {
    tracing::debug!("establishing client connection...");

    // Split the socket into a sender and receive of messages.
    let (ws_tx, mut ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages to the socket
    let (tx, rx) = unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(rx.forward(ws_tx).map(|result| {
        if let Err(e) = result {
            tracing::error!("error sending websocket message: {}", e);
        }
    }));

    // Store client
    let client = Client::new(tx.clone());
    tracing::info!("client connected with id: {}", client.id.clone());

    service.lock().await.on_connect(&client.clone()).await;

    receive_messages(&mut ws_rx, client.clone(), &service).await;

    service.lock().await.on_disconnect(&client.clone()).await;
}

pub async fn broadcast_message(msg: Message, clients: &Clients) {
    for (_id, client) in clients.into_iter() {
        tracing::debug!("sending broadcast message");
        client.send_message(msg.clone()).await;
    }
    return;
}

// Private helpers
async fn receive_messages(
    ws_rx: &mut SplitStream<WebSocket>,
    client: Client,
    service: &WebSocketService,
) {
    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(err) => {
                tracing::error!("error receiving message from client {}: {}", client.id, err);
                break;
            }
        };
        handle_received_message(&client, msg, &service).await;
    }
}

async fn handle_received_message(client: &Client, msg: Message, service: &WebSocketService) {
    tracing::debug!("received message from {}: {:?}", client.id.clone(), msg);

    match msg.clone() {
        Message::Text(text) => {
            if handle_received_ping(text, &client).await {
                return;
            }
            // TODO: call on_message() on client handler
            service.lock().await.on_message(&client.clone()).await;
        }
        Message::Close(_) => service.lock().await.on_disconnect(&client.clone()).await,
        _ => {}
    }
}

async fn handle_received_ping(msg: String, client: &Client) -> bool {
    if msg.trim().to_lowercase() == "ping" {
        tracing::debug!("replying to PING from {} with PONG", client.id);
        client
            .send_message(Message::Text(String::from("PONG")))
            .await;
        return true;
    }
    false
}
