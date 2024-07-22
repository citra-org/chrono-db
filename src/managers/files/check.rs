use std::path::Path;
use std::io::Result;

pub fn check_file(path: &str) -> Result<()> {
    let path = Path::new(path);

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
