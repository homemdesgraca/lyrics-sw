use std::{error::Error, path::{PathBuf}};

use clap::Parser;

mod input_handler;
mod library_handler;
mod requests_handler;
mod declarations;

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
        eprintln!("'{}' is not a valid path.", library_path.to_string_lossy());
        return Ok(());
    }
    
    let all_songs: Vec<PathBuf> = library_handler::get_files(library_path)?;

    let missing_lyrics: Vec<PathBuf> = library_handler::get_missing_lyrics(all_songs);

    requests_handler::request_lyrics(missing_lyrics)?;

    println!("Done.");

    Ok(())

}