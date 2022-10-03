use super::{radio::StationService, ws::handle_client_connection};
use axum::{
    extract::{State, WebSocketUpgrade},
    response::{Html, IntoResponse},
};

// Include utf-8 file at **compile** time.
pub async fn index_handler() -> Html<&'static str> {
    Html(std::include_str!("../../frontend/index.html"))
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(station): State<StationService>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_client_connection(socket, station))
}
