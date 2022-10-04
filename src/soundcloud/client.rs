use super::{Playlist, Track};
use crate::error::Error;
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{header::HeaderMap, Response};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::Deserialize;
use serde_json::Value;

static USER_AGENT: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:100.0) Gecko/20100101 Firefox/100.0";

pub async fn fetch_playlist_tracks(client_id: &str, playlist_id: &str) -> Result<Playlist, Error> {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", USER_AGENT.parse().unwrap());

    let url = format!(
        "https://api-v2.soundcloud.com/playlists/{}?client_id={}",
        playlist_id, client_id
    );

    let res = http_get(url.as_str(), &headers).await?;
    match res.json::<PlaylistResponse>().await {
        Ok(res) => Ok(res.into()),
        Err(_) => Err(Error::SoundcloudJsonParseError(String::from(
            "PlaylistResponse",
        ))),
    }
}

pub async fn fetch_track_info(client_id: &str, track_id: u64) -> Result<Track, Error> {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", USER_AGENT.parse().unwrap());

    let url = format!(
        "https://api-v2.soundcloud.com/tracks/{}?client_id={}",
        track_id, client_id
    );

    let res = http_get(url.as_str(), &headers).await?;
    match res.json::<TrackResponse>().await {
        Ok(res) => Ok(res.try_into()?),
        Err(_) => Err(Error::SoundcloudJsonParseError(String::from(
            "TrackResponse",
        ))),
    }
}

pub async fn fetch_track_stream(
    client_id: &str,
    track_url: &str,
    token: &str,
) -> Result<TrackStreamResponse, Error> {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", USER_AGENT.parse().unwrap());
    headers.insert("Authorization", format!("Oauth {}", token).parse().unwrap());

    let url = format!("{}?client_id={}", track_url, client_id);

    let res = http_get(url.as_str(), &headers).await?;
    match res.json::<TrackStreamResponse>().await {
        Ok(res) => Ok(res),
        Err(_) => Err(Error::SoundcloudJsonParseError(String::from(
            "TrackResponse",
        ))),
    }
}

pub async fn fetch_new_client_id() -> Result<String, Error> {
    let url = "https://soundcloud.com";
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", USER_AGENT.parse().unwrap());

    let content = http_get(url, &headers)
        .await?
        .text_with_charset("utf-8")
        .await
        .map_err(Error::SoundcloudTextResponseError)?;

    find_client_id(content).await
}

async fn find_client_id(page: String) -> Result<String, Error> {
    lazy_static! {
        static ref RE_SRC: Regex = Regex::new(r#"<script[^>]+src="([^"]+)""#).unwrap();
        static ref RE_CLIENT_ID: Regex =
            Regex::new(r#"client_id\s*:\s*"([0-9a-zA-Z]{32})""#).unwrap();
    }

    for src in RE_SRC.captures_iter(page.as_str()) {
        let url = &src[1];

        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", USER_AGENT.parse().unwrap());

        let js = http_get(url, &headers)
            .await?
            .text_with_charset("utf-8")
            .await
            .map_err(Error::SoundcloudTextResponseError)?;

        if let Some(cap) = RE_CLIENT_ID.captures(js.as_str()) {
            let client_id = cap.get(1).map_or("", |m| m.as_str());
            tracing::info!("fetched new client id {}", client_id);
            return Ok(client_id.to_string());
        }
    }

    tracing::warn!("unable to fetch new client id");
    Err(Error::SoundcloudClientIdUpdateError(String::from(
        "SoundCloud client_id not found",
    )))
}

async fn http_get(url: &str, headers: &HeaderMap) -> Result<Response, Error> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let res = client
        .get(url)
        .headers(headers.clone())
        .send()
        .await
        .map_err(Error::SoundcloudRequestError)?;

    if !res.status().is_success() {
        let err = Error::SoundcloudResponseError(res.status().as_u16());
        tracing::error!("{} requesting `{}`", err, url);
        return Err(err);
    }

    Ok(res)
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaylistResponse {
    #[serde(rename = "artwork_url")]
    pub artwork_url: Option<Value>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub description: String,
    pub duration: u64,
    #[serde(rename = "embeddable_by")]
    pub embeddable_by: String,
    pub genre: String,
    pub id: u64,
    pub kind: String,
    #[serde(rename = "label_name")]
    pub label_name: String,
    #[serde(rename = "last_modified")]
    pub last_modified: String,
    pub license: String,
    #[serde(rename = "likes_count")]
    pub likes_count: u64,
    #[serde(rename = "managed_by_feeds")]
    pub managed_by_feeds: bool,
    pub permalink: String,
    #[serde(rename = "permalink_url")]
    pub permalink_url: String,
    pub public: bool,
    #[serde(rename = "purchase_title")]
    pub purchase_title: Option<Value>,
    #[serde(rename = "purchase_url")]
    pub purchase_url: Option<Value>,
    #[serde(rename = "release_date")]
    pub release_date: Option<Value>,
    #[serde(rename = "reposts_count")]
    pub reposts_count: u64,
    #[serde(rename = "secret_token")]
    pub secret_token: Option<Value>,
    pub sharing: String,
    #[serde(rename = "tag_list")]
    pub tag_list: String,
    pub title: String,
    pub uri: String,
    #[serde(rename = "user_id")]
    pub user_id: u64,
    #[serde(rename = "set_type")]
    pub set_type: String,
    #[serde(rename = "is_album")]
    pub is_album: bool,
    #[serde(rename = "published_at")]
    pub published_at: String,
    #[serde(rename = "display_date")]
    pub display_date: String,
    pub user: User,
    pub tracks: Vec<TrackResponse>,
    #[serde(rename = "track_count")]
    pub track_count: u64,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TrackResponse {
    #[serde(rename = "artwork_url")]
    pub artwork_url: Option<String>,
    pub caption: Option<Value>,
    pub commentable: Option<bool>,
    #[serde(rename = "comment_count")]
    pub comment_count: Option<u64>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub downloadable: Option<bool>,
    #[serde(rename = "download_count")]
    pub download_count: Option<u64>,
    pub duration: Option<u64>,
    #[serde(rename = "full_duration")]
    pub full_duration: Option<u64>,
    #[serde(rename = "embeddable_by")]
    pub embeddable_by: Option<String>,
    pub genre: Option<String>,
    #[serde(rename = "has_downloads_left")]
    pub has_downloads_left: Option<bool>,
    pub id: u64,
    pub kind: String,
    #[serde(rename = "label_name")]
    pub label_name: Option<String>,
    #[serde(rename = "last_modified")]
    pub last_modified: Option<String>,
    pub license: Option<String>,
    #[serde(rename = "likes_count")]
    pub likes_count: Option<u64>,
    pub permalink: Option<String>,
    #[serde(rename = "permalink_url")]
    pub permalink_url: Option<String>,
    #[serde(rename = "playback_count")]
    pub playback_count: Option<u64>,
    pub public: Option<bool>,
    #[serde(rename = "publisher_metadata")]
    pub publisher_metadata: Option<Value>,
    #[serde(rename = "purchase_title")]
    pub purchase_title: Option<String>,
    #[serde(rename = "purchase_url")]
    pub purchase_url: Option<String>,
    #[serde(rename = "release_date")]
    pub release_date: Option<Value>,
    #[serde(rename = "reposts_count")]
    pub reposts_count: Option<u64>,
    #[serde(rename = "secret_token")]
    pub secret_token: Option<Value>,
    pub sharing: Option<String>,
    pub state: Option<String>,
    pub streamable: Option<bool>,
    #[serde(rename = "tag_list")]
    pub tag_list: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "track_format")]
    pub track_format: Option<String>,
    pub uri: Option<String>,
    pub urn: Option<String>,
    #[serde(rename = "user_id")]
    pub user_id: Option<u64>,
    pub visuals: Option<Value>,
    #[serde(rename = "waveform_url")]
    pub waveform_url: Option<String>,
    #[serde(rename = "display_date")]
    pub display_date: Option<String>,
    pub media: Option<Media>,
    #[serde(rename = "station_urn")]
    pub station_urn: Option<String>,
    #[serde(rename = "station_permalink")]
    pub station_permalink: Option<String>,
    #[serde(rename = "track_authorization")]
    pub track_authorization: Option<String>,
    #[serde(rename = "monetization_model")]
    pub monetization_model: String,
    pub policy: String,
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TrackStreamResponse {
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct User {
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "followers_count")]
    pub followers_count: u64,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub id: u64,
    pub kind: String,
    #[serde(rename = "last_modified")]
    pub last_modified: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    pub permalink: String,
    #[serde(rename = "permalink_url")]
    pub permalink_url: String,
    pub uri: String,
    pub urn: String,
    pub username: String,
    pub verified: bool,
    pub city: Option<Value>,
    #[serde(rename = "country_code")]
    pub country_code: Option<Value>,
    pub badges: Badges,
    #[serde(rename = "station_urn")]
    pub station_urn: String,
    #[serde(rename = "station_permalink")]
    pub station_permalink: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Badges {
    pub pro: bool,
    #[serde(rename = "pro_unlimited")]
    pub pro_unlimited: bool,
    pub verified: bool,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Media {
    pub transcodings: Vec<Transcoding>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Transcoding {
    pub url: String,
    pub preset: String,
    pub duration: u64,
    pub snipped: bool,
    pub format: Format,
    pub quality: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Format {
    pub protocol: String,
    #[serde(rename = "mime_type")]
    pub mime_type: String,
}
