use std::path::{Path, PathBuf};

pub fn normalize_path(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        let home = std::env::var("HOME").unwrap();
        let path = path.strip_prefix("~/").unwrap();
        Path::new(&home).join(path)
    } else {
        PathBuf::from(path)
    }
}
