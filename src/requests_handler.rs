use crate::declarations::*;
use crate::library_handler;
use crate::logging::*;

use std::path::PathBuf;
use std::error::Error;

use reqwest::{blocking as request};

use lofty::tag::Tag;
use lofty::{prelude::*};
use lofty::{read_from_path};

use urlencoding::encode;

const LRCLIB_URL: &str = "https://lrclib.net";

pub fn request_lyrics(songs: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {

    let client = build_client()?;

    for song in songs {

        let song_entry = match read_from_path(&song) {
            Ok(value) => value,
            Err(err) => {
                log_error(err);
                continue;
            }
        };

        let properties = match song_entry.primary_tag() {
            Some(tagged) => get_properties(tagged),
            None => {
                log_error_display(format!("{} is not properly tagged (missing title, artist or album.", song.to_string_lossy()));
                continue;
            }
        };
        
        let (track_name, artist, album) = properties;
        let duration = song_entry.properties().duration().as_secs();

        log_warn(format!("Requesting lyrics for {} by {}...", track_name, artist));

        let get_request = construct_get_request(&track_name, &artist, &album, &duration);

        log_debug_display(format!("Request sent to lrclib: {}", get_request));

        let response = match client.get(get_request).send() {
            Ok(value) => value,
            Err(err) => {
                log_error(err);
                continue;
            }
        };

        let response_lyrics: LyricsResponse = match response.json::<LyricsResponse>() {
            Ok(value) => value,
            Err(err) => {
                log_error(err);
                continue;
            }
        };

        log_debug_display(format!("Raw JSON response from lrclib: {:#?}", response_lyrics));

        let lyrics = match &response_lyrics.synced_lyrics {
            Some(value) => {
                        log_info(format!("Found synced lyrics for {} by {}!", track_name, artist));
                        value
                    }
            None => {
                log_warn(format!("Couldn't find synced lyrics for {} by {}. Trying unsynced lyrics...", track_name, artist));
                match &response_lyrics.plain_lyrics {
                    Some(value) => {
                        log_info(format!("Found unsynced lyrics for {} by {}!", track_name, artist));
                        value
                    },
                    None => {
                        log_error_display(format!("Couldn't find lyrics for {} by {}. Ignoring...", track_name, artist));
                        continue;
                    }
            }}
        };

        match library_handler::write_lrc(&song, lyrics) {
            Ok(_) => {
                log_info(format!("Successfully wrote a .lrc file alongside {}.", song.to_string_lossy()));
                increment_success_counter();
            },
            Err(err) => {
                log_error(err);
            },
        }

    }

    Ok(())

}

fn build_client() -> Result<request::Client, reqwest::Error> {
    request::Client::builder()
    .user_agent(USER_AGENT)
    .build()
}

fn get_properties(song_tagged: &Tag) -> (String, String, String) {
    (
        song_tagged.title().unwrap_or_default().into_owned(),
        song_tagged.artist().unwrap_or_default().into_owned(),
        song_tagged.album().unwrap_or_default().into_owned(),
    )
}

fn construct_get_request(track_name: &str, artist: &str, album: &str, duration_secs: &u64) -> String {

    let request = format!(
    "{LRCLIB_URL}/api/get?track_name={}&artist_name={}&album_name={}&duration={}",
    encode(track_name),
    encode(artist),
    encode(album),
    duration_secs,
    );

    request
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_request_construction() {
        let title_input = "test: track";
        let artist_input = "test, artist & testing ! | hi |";
        let album_input = "TESTING: Test Album";
        let duration_input: u64 = 64;
        let request_result = construct_get_request(title_input, artist_input, album_input, &duration_input);

        let expected_result = format!("{LRCLIB_URL}/api/get?track_name={}&artist_name={}&album_name={}&duration={}", encode(title_input), encode(artist_input), encode(album_input), duration_input);

        assert_eq!(request_result, expected_result);
    }
}