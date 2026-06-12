use std::error::Error;

use librespot::core::SpotifyUri;
use librespot::metadata::Metadata;
use librespot::oauth::OAuthClientBuilder;
use librespot::oauth::OAuthClient;
use librespot::core::{SessionConfig, authentication::Credentials, session::Session};
use librespot::oauth::OAuthToken;
use librespot::metadata::{Playlist, Track};

use crate::config::SCOPES;
use crate::config::Config;
use crate::recorder::record_track;
use crate::track::TrackMeta;


async fn get_access_token(config: Config) -> OAuthToken {
    let scopes: Vec<&str> = SCOPES.to_vec();
    let builder: OAuthClientBuilder = OAuthClientBuilder::new(config.client_id.trim(), config.redirect_uri.trim(), scopes);        
    let client: OAuthClient = builder.build().unwrap();
    return client.get_access_token().unwrap()
}


pub async fn extract_track(track_id: &SpotifyUri, session: &Session) -> Result<TrackMeta, Box<dyn Error>> {
    let track: Track = Track::get(session, track_id).await.unwrap();

    let track_name = track.name;
    let album_name = track.album.name;
    let track_number = track.number;
    let artists_names: Vec<String> = track.artists.iter().map(|artist| artist.name.clone()).collect();
    let album_artists_names: Vec<String> = track.album.artists.iter().map(|artist| artist.name.clone()).collect();
    let date = track.album.date;
    let image_url = format!("https://i.scdn.co/image/{}", track.album.covers.first().unwrap().id);    
    
    println!("Track: {}", track_name);
    println!("Track #: {}", track_number);
    for artist_name in artists_names.clone() { 
        println!("Artists: {}", artist_name)
    }
    println!("Album: {}", album_name);
    println!("Date: {}", date.to_string());
    println!("Image URL: {}", image_url);
    Ok(
        TrackMeta{
            id: track.id, 
            name: track_name, 
            album_name: album_name, 
            number: track_number, 
            artists: artists_names, 
            album_artists: album_artists_names, 
            date: date, 
            image_url: image_url
        }
    )    
}

pub async fn extract_playlist(config: Config, playlist_uri: String) -> Result<(), Box<dyn Error>> {
    println!("start extracting playlist...");
    let token = get_access_token(config).await;

    let session: Session = Session::new(SessionConfig::default(), None);
    let credentials: Credentials = Credentials::with_access_token(token.access_token);
    session.connect(credentials, true).await?;
    println!("successfully connected to spotify session...");
    
    let playlist_id = SpotifyUri::from_uri(playlist_uri.as_str()).unwrap();
    let playlist: Playlist = Playlist::get(&session, &playlist_id).await?;
    
    for i in 0..playlist.contents.items.len() {
        let track_item = &playlist.contents.items[i];
        let track_id = &track_item.id;
        let track_meta = extract_track(track_id, &session).await?;
        record_track(session.clone(), track_meta).await;
    } 

    Ok(())

}
