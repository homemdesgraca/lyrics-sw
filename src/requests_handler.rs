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

const LRCLIB_URL: &str = "https://lrclib.net/";

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
                log_error(format!("{} is not properly tagged (missing title, artist or album.", song.to_string_lossy()));
                continue;
            }
        };
        
        let (track_name, artist, album) = properties;
        let duration = song_entry.properties().duration().as_secs();

        log_warn(format!("Requesting lyrics for {} by {}...", track_name, artist));

        let get_request = construct_get_request(&track_name, &artist, &album, &duration);

        let response = client.get(get_request).send()?;

        let response_lyrics: LyricsResponse = response.json()?;

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
                        log_error(format!("Couldn't find lyrics for {} by {}. Ignoring...", track_name, artist));
                        continue;
                    }
            }}
        };

        match library_handler::write_lrc(&song, lyrics) {
            Ok(_) => {
                log_info(format!("Successfully wrote a .lrc file alongside {}.", song.to_string_lossy()));
                increment_success_counter();
            },
            Err(err) => log_error(err),
        }

    }

    Ok(())

}

fn build_client() -> Result<request::Client, reqwest::Error> {
    request::Client::builder()
    .user_agent("lyrics-sw (https://github.com/homemdesgraca/lyrics-sw)")
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
    "{LRCLIB_URL}/api/get?artist_name={}&track_name={}&album_name={}&duration={}",
    encode(artist),
    encode(track_name),
    encode(album),
    duration_secs,
    );

    request

}