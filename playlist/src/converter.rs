use crate::playlist::Platform;
use std::error::Error;

pub struct PlaylistConverter;

impl PlaylistConverter {
    pub fn convert_playlist(
        source: &dyn Platform,
        target: &dyn Platform,
        playlist_id: &str,
        new_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        // Fetch tracks from source platform
        let tracks = source.get_playlist_tracks(playlist_id)?;
        println!("Tracks fetched: {:?}", tracks);

        // Create playlist on the target platform
        let new_playlist_id = target.create_playlist(new_name, &tracks)?;
        println!(
            "Successfully created new playlist on target platform with ID: {}",
            new_playlist_id
        );

        Ok(())
    }
}
