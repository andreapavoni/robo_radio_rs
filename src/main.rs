use axum::{
    http::{header, HeaderValue},
    routing::get,
    Router,
};
use axum_extra::routing::SpaRouter;
use robo_radio::{
    error::Error,
    web::{
        handlers::{index_handler, websocket_handler},
        radio::{go_live, Station, StationService},
    },
};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::{
    set_header::SetResponseHeaderLayer,
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing_subscriber::{EnvFilter, FmtSubscriber, filter::LevelFilter};

#[tokio::main]
async fn main() -> Result<(), Error> {
    FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into())
        )
        .with_target(true)
        .with_ansi(true)
        .compact()
        .init();

    let playlist_id = match env::var_os("ROBO_RADIO_SOUNDCLOUD_PLAYLIST_ID") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$ROBO_RADIO_SOUNDCLOUD_PLAYLIST_ID is not set"),
    };

    let station = Station::new(playlist_id.as_str()).await?;
    let station_service: StationService = Arc::new(Mutex::new(station));

    let app = Router::with_state(station_service.clone())
        .route("/", get(index_handler))
        .route("/ws", get(websocket_handler))
        .merge(SpaRouter::new("/assets", "assets"))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("max-age=3600"),
        ))
        .layer(
            TraceLayer::new_for_http().on_response(
                DefaultOnResponse::new()
                    .include_headers(true)
                    .latency_unit(LatencyUnit::Micros),
            ),
        );

    tokio::spawn(async move {
        go_live(station_service.clone()).await;
    });

    // Use "[::]" to listen on both IPv4 (0.0.0.0) and IPv6
    let srv_host = env::var("ROBO_RADIO_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let srv_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    tracing::info!("address listening `{}`:`{}`", srv_host, srv_port);

    let addr = format!("{}:{}", srv_host, srv_port)
        .parse::<SocketAddr>()
        .expect(format!("unable to parse socket address with `{}:{}`", srv_host, srv_port).as_str());

    tracing::info!("server started and listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
