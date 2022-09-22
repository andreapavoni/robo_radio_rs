use axum::{extract::Extension, routing::get, Router};
use axum_extra::routing::SpaRouter;
use robo_radio_rs::{
    error::Error,
    web::{
        handlers::{index_handler, websocket_handler},
        radio::{go_live, new_station},
    },
};
use std::env;

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

    let station = new_station(client_id, playlist_id).await?;

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/socket/websocket", get(websocket_handler))
        .merge(SpaRouter::new("/assets", "assets"))
        .layer(Extension(station.clone()));

    tokio::spawn(async move {
        go_live(station.clone()).await;
    });

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
