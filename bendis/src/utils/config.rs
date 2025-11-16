use std::path::PathBuf;

/// Get the .bendis directory path
pub fn get_bendis_dir() -> PathBuf {
    PathBuf::from(".bendis")
}

/// Get the project root directory path
pub fn get_root_dir() -> PathBuf {
    PathBuf::from(".")
}
