use crate::managers;
use std::fs;
use std::io::Result;
use std::path::{Path,PathBuf};

pub fn create_file(path: &str, filename: &str, is_root: bool) -> Result<()> {
    managers::folders::check::check_folder(path, is_root)?;
    let full_path: PathBuf;
    if is_root {
        let initial_path = Path::new("/var/lib/citra/chrono");
        let base_path:PathBuf = initial_path.join(path);
        full_path = base_path.join(filename);
    } else {
        full_path = Path::new(filename).to_path_buf();
    }

    fs::File::create(&full_path)?;
    println!("File created successfully.");

    Ok(())
}
