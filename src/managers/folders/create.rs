use std::fs;
use std::io::Result;
use std::path::PathBuf;

pub fn create_folder(folder: &str) -> Result<()> {
    let base_path = PathBuf::from(folder);

    fs::create_dir_all(&base_path)?;
    println!("Folder created successfully at {:?}", base_path);

    Ok(())
}
