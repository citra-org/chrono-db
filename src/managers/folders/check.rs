use std::io::Result;
use std::path::{Path, PathBuf};

pub fn check_folder(folder: &str, is_root: bool) -> Result<()> {
    let path: PathBuf;
    if is_root {
        let base_path = Path::new("/var/lib/citra/chrono");
        path = base_path.join(folder);
    } else {
        path = Path::new(folder).to_path_buf();
    }

    if path.exists() {
        if path.is_dir() {
            println!("Directory '{}' exists.", path.display());
        } else {
            println!("Path '{}' exists but is not a directory.", path.display());
        }
    } else {
        println!("Path '{}' does not exist.", path.display());
    }

    Ok(())
}
