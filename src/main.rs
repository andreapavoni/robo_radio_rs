use axum::{routing::get, Router};
use std::env;

use robo_radio_rs::{error::Error, media_player::MediaPlayer, soundcloud::ApiClient};

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

    media_player.load_playlist(playlist_id).await?;
    media_player.load_next_track().await?;

    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
