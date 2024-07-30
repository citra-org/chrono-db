use std::io::Result;
use std::path::{Path, PathBuf};

pub fn check_file(file: &str, is_root: bool) -> Result<()> {
    let path: PathBuf;
    if is_root {
        let base_path = Path::new("/var/lib/citra/chrono");
        path = base_path.join(file);
    } else {
        path = Path::new(file).to_path_buf();
    }

    if path.exists() {
        if path.is_file() {
            println!("File '{}' exists.", path.display());
        } else {
            println!("Path '{}' exists but is not a file.", path.display());
        }
    } else {
        println!("Path '{}' does not exist.", path.display());
    }

    Ok(())
}
