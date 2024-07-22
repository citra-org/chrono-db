use std::fs;
use std::path::Path;
use std::io::Result;

pub fn create_folder(foldername: &str, overwrite: bool) -> Result<()> {
    let path = Path::new(foldername);

    if path.exists() {
        if overwrite {
            fs::remove_dir_all(path)?;
            println!("existing folder removed");
        } else {
            println!("folder already exists");
            return Ok(());
        }
    }

    fs::create_dir(path)?;
    println!("folder created successfully");

    Ok(())
}
