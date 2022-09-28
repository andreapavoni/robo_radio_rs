use axum::{extract::Extension, routing::get, Router};
use axum_extra::routing::SpaRouter;
use robo_radio_rs::{
    error::Error,
    web::{
        handlers::{index_handler, websocket_handler},
        radio::{go_live, new_station},
    },
};
use std::{env, net::SocketAddr};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "robo_radio_rs=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client_id = match env::var_os("ROBO_RADIO_SOUNDCLOUD_CLIENT_ID") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$ROBO_RADIO_SOUNDCLOUD_CLIENT_ID is not set"),
    };

    let playlist_id = match env::var_os("ROBO_RADIO_SOUNDCLOUD_PLAYLIST_ID") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$ROBO_RADIO_SOUNDCLOUD_PLAYLIST_ID is not set"),
    };

    let station = new_station(client_id, playlist_id).await?;

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(websocket_handler))
        .merge(SpaRouter::new("/assets", "assets"))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(station.clone()));

    tokio::spawn(async move {
        go_live(station.clone()).await;
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("server started and listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
