use axum::{
    extract::WebSocketUpgrade,
    response::{Html, IntoResponse},
    Extension,
};

use super::{radio::Station, socket::websocket_connection};

// Include utf-8 file at **compile** time.
pub async fn index_handler() -> Html<&'static str> {
    Html(std::include_str!("../../frontend/index.html"))
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(station): Extension<Station>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket_connection(socket, station))
}
