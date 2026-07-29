use std::path::{Path};

pub fn is_valid_path(path: &Path) -> bool { 
    path.is_dir()
}