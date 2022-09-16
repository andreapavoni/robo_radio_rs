mod api_data;
pub mod client;

#[derive(Debug)]
pub struct Playlist {
    pub tracks_ids: Vec<u64>,
}

impl std::fmt::Display for Playlist {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.tracks_ids)
    }
}

#[derive(Debug)]
pub struct Track {
    pub id: u64,
    pub permalink_url: Option<String>,
    pub artwork_url: Option<String>,
    pub duration: Option<u64>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub artist_permalink: Option<String>,
    pub url: Option<String>,
    pub token: Option<String>,
}
