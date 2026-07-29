use std::{io};
use std::path::PathBuf;

pub fn get_path() -> PathBuf {
    loop {
        let mut library_path = String::new();

        println!("Enter the path to your music library:");

        match io::stdin().read_line(&mut library_path) {
            Ok(bytes) => {
                if bytes <= 1 {
                    println!("Your path can't be empty.");
                    library_path.clear();
                } else {
                    let path = PathBuf::from(library_path.trim());

                    if path.is_dir() {
                        break path;
                    } else {
                        println!("'{}' isn't a directory.", library_path.trim());
                        library_path.clear();
                    };
                };
            },
            Err(err) => {
                println!("Error: {}", err);
                library_path.clear();
                continue;
            }
        };
    }
}