use axum::{routing::get, Router};
use std::env;

use robo_radio_rs::soundcloud::client::{get_playlist_tracks, get_track};

#[tokio::main]
async fn main() {
    let client_id = match env::var_os("ROBO_RADIO_SOUNDCLOUD_CLIENT_ID") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$ROBO_RADIO_SOUNDCLOUD_CLIENT_ID is not set"),
    };

    let playlist_id = match env::var_os("ROBO_RADIO_SOUNDCLOUD_PLAYLIST_ID") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$ROBO_RADIO_SOUNDCLOUD_PLAYLIST_ID is not set"),
    };

    match get_playlist_tracks(client_id.clone(), playlist_id).await {
        Ok(playlist_ids) => println!("====== Playlist IDs {:?}", playlist_ids),
        Err(err) => println!("====== Error Get Playlist IDs {:?}", err),
    };

    let track_id = 142863571;

    match get_track(client_id, track_id).await {
        Ok(track) => println!("====== Track {:?}", track),
        Err(err) => println!("====== Error Get Track {:?}", err),
    };

    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
