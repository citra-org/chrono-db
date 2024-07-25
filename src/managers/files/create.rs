use crate::managers;
use std::fs;
use std::io::Result;
use std::path::Path;

pub fn create_file(path: &str, filename: &str, overwrite: bool) -> Result<()> {
    managers::folders::check::check_folder(path)?;
    let full_path = Path::new(path).join(filename);

    if full_path.exists() {
        if overwrite {
            fs::remove_file(&full_path)?;
            println!("Existing file removed.");
        } else {
            println!("File already exists.");
            return Ok(());
        }
    }
    fs::File::create(&full_path)?;
    println!("File created successfully.");

    Ok(())
}
