use dotenvy::dotenv;
use playlist::converter::PlaylistConverter;
use playlist::playlist::{AppleMusic, Platform, Spotify, YoutubeMusic};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the environment variables
    let youtube_api_key = env::var("YOUTUBE_API_KEY")?;

    let youtube_music = YoutubeMusic {
        auth_token: youtube_api_key,
    };

    let spotify = Spotify {
        api_key: "spotify_api_key".to_string(),
    };
    let apple_music = AppleMusic {
        auth_token: "apple_auth_token".to_string(),
    };

    println!("{:?}", &youtube_music);
    println!(
        "{:?}",
        &youtube_music.get_playlist_tracks("PL-ocF3xlni7RoKshc5W6IRBGTKrSrlzVr")?
    );
    Ok(())
}
