use librespot::core::{SpotifyUri, date::Date};


#[derive(Clone)]
pub struct TrackMeta {
    pub id: SpotifyUri,
    pub name: String,
    pub album_name: String,
    pub number: i32,
    pub artists: Vec<String>,
    pub album_artists: Vec<String>,
    pub date: Date,
    pub image_url: String,
}
