use std::{error::Error, path::{PathBuf}};

mod input_handler;
mod library_handler;
mod requests_handler;
mod declarations;

fn main() -> Result<(), Box<dyn Error>> {

    let library_path = input_handler::get_path();
    
    let all_songs: Vec<PathBuf> = library_handler::get_files(library_path)?;

    let missing_lyrics: Vec<PathBuf> = library_handler::get_missing_lyrics(all_songs);

    requests_handler::request_lyrics(missing_lyrics)?;

    println!("Done.");

    Ok(())

}