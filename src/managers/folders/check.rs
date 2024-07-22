use std::path::Path;
use std::io::Result;

pub fn check_folder(path: &str) -> Result<()> {
    let path = Path::new(path);

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
