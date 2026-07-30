use logging::*;
use crate::declarations::*;

use std::{error::Error, path::{PathBuf}};

use clap::Parser;

mod input_handler;
mod library_handler;
mod requests_handler;
mod declarations;
mod logging;

/// lyrics-sw is a minimal, Rust-based CLI tool to fetch lyrics for your local music library.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to music library
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {

    let args = Args::parse();

    let library_path = args.path;

    if !input_handler::is_valid_path(&library_path) {
        let log_text = format!("'{}' is not a valid directory.", library_path.to_string_lossy());
        log_error(log_text);
        return Ok(());
    }

    log_info("Starting library scan...");
    
    let all_songs: Vec<PathBuf> = library_handler::get_files(library_path)?;

    log_info("Library successfully scanned.");
    log_info(format!("{} songs found.", all_songs.len()));

    let missing_lyrics: Vec<PathBuf> = library_handler::get_missing_lyrics(all_songs);

    line_break();
    log_warn(format!("Searching for lyrics for {} songs...", missing_lyrics.len()));

    requests_handler::request_lyrics(missing_lyrics)?;

    line_break();
    log_info(format!("Done! Wrote .lrc files for {} songs", get_success_counter()));

    Ok(())

}