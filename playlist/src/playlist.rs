use reqwest::blocking::Client;
use serde_json::Value;
use std::{collections::HashMap, error::Error};

// Define the Platform trait
pub trait Platform {
    fn get_playlist_tracks(&self, playlist_id: &str) -> Result<Vec<String>, Box<dyn Error>>;
    fn create_playlist(&self, name: &str, tracks: &[String]) -> Result<String, Box<dyn Error>>;
}

// Spotify implementation
pub struct Spotify {
    pub api_key: String,
}

impl Platform for Spotify {
    fn get_playlist_tracks(&self, playlist_id: &str) -> Result<Vec<String>, Box<dyn Error>> {
        println!("Fetching tracks from Spotify playlist: {}", playlist_id);
        Ok(vec!["Song A".to_string(), "Song B".to_string()])
    }

    fn create_playlist(&self, name: &str, tracks: &[String]) -> Result<String, Box<dyn Error>> {
        println!(
            "Creating Spotify playlist '{}' with {} tracks.",
            name,
            tracks.len()
        );
        Ok("spotify_playlist_id_123".to_string())
    }
}

// Apple Music implementation
pub struct AppleMusic {
    pub auth_token: String,
}

impl Platform for AppleMusic {
    fn get_playlist_tracks(&self, playlist_id: &str) -> Result<Vec<String>, Box<dyn Error>> {
        println!("Fetching tracks from Apple Music playlist: {}", playlist_id);
        Ok(vec!["Song C".to_string(), "Song D".to_string()])
    }

    fn create_playlist(&self, name: &str, tracks: &[String]) -> Result<String, Box<dyn Error>> {
        println!(
            "Creating Apple Music playlist '{}' with {} tracks.",
            name,
            tracks.len()
        );
        Ok("apple_music_playlist_id_456".to_string())
    }
}

#[derive(Debug)]
pub struct YoutubeMusic {
    pub auth_token: String,
}

impl Platform for YoutubeMusic {
    fn get_playlist_tracks(&self, playlist_id: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let client = Client::new();
        let mut tracks = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let mut params = HashMap::new();
            params.insert("part", "snippet");
            params.insert("playlistId", playlist_id);
            params.insert("key", &self.auth_token);
            params.insert("maxResults", "5");

            if let Some(token) = &page_token {
                params.insert("pageToken", token);
            }

            let response = client
                .get("https://www.googleapis.com/youtube/v3/playlistItems")
                .query(&params)
                .send()?;
            let json: Value = response.json()?;

            if let Some(items) = json["items"].as_array() {
                for item in items {
                    if let Some(title) = item["snippet"]["title"].as_str() {
                        tracks.push(title.to_string());
                    }
                }
            }

            // Check for nextPageToken
            page_token = json["nextPageToken"].as_str().map(|s| s.to_string());

            if page_token.is_none() {
                break;
            }
        }

        println!(
            "Fetched {} tracks from YouTube playlist: {}",
            tracks.len(),
            playlist_id
        );
        Ok(tracks)
    }
    fn create_playlist(&self, name: &str, tracks: &[String]) -> Result<String, Box<dyn Error>> {
        println!(
            "Creating Youtube Music playlist '{}' with {} tracks.",
            name,
            tracks.len()
        );
        Ok("youtube_music_playlist_id_789".to_string())
    }
}
