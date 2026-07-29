use std::error::Error;
use std::path::{Path, PathBuf};
use lofty::file::EXTENSIONS;
use lofty::{prelude::*};
use walkdir::WalkDir;
use lofty::read_from_path;

pub fn get_files(path: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {

    let mut file_list: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        match entry.path().extension() {
            Some(ext) => {
                if EXTENSIONS.contains(&ext.to_string_lossy().as_ref()) {
                    file_list.push(entry.path().to_path_buf());
                }
            }
            None => continue
        }
    }

    Ok(file_list)
}

pub fn get_missing_lyrics(songs: Vec<PathBuf>) -> Vec<PathBuf> {

    let mut missing_lyrics: Vec<PathBuf> = Vec::new();

    for song in songs {

        if lrc_check(&song) {
            continue;
        }

        let song_entry = match read_from_path(&song) {
            Ok(value) => value,
            Err(_) => continue
        };

        if let Some(tagged) = song_entry.primary_tag() {
            match tagged.get_string(ItemKey::Lyrics) {
                Some(_) => continue,
                None => missing_lyrics.push(song)
            }
        }
    }

    missing_lyrics
}

fn lrc_check(song: &Path) -> bool {
    song.with_extension("lrc").is_file()
}