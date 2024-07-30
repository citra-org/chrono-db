use crate::managers;
use std::fs;
use std::io::Result;
use std::path::Path;

pub fn delete_file(path: &str, filename: &str) -> Result<()> {
    let full_path = Path::new(path).join(filename);

    let _ = managers::files::check::check_file(&full_path.to_string_lossy(), true);

    if full_path.exists() {
        fs::remove_file(&full_path)?;
        println!("File '{}' deleted successfully.", filename);
    } else {
        println!("File '{}' does not exist or is not a file.", filename);
    }

    Ok(())
}
