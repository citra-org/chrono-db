use crate::assist;
use std::fs::{self};
use std::io::Result;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const DEFAULT_KEEPER: &str = "admin";

pub fn create_keeper(chrono: &str, keeper: Option<&str>) -> Result<()> {
    let full_path: &str = &format!("/home/kali/.citra/chrono/{}/config", chrono);
    let dir_path = std::path::Path::new(&full_path).parent().unwrap();

    fs::create_dir_all(dir_path)?;
    fs::File::create(full_path)?;

    let content = match fs::read_to_string(&full_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            String::new()
        }
    };

    if content.trim().is_empty() {
        let user = keeper.unwrap_or(DEFAULT_KEEPER);
        let password = assist::password::generate_random_password(16);

        let new_keeper = format!("{}:{}", user, password);
        fs::write(&full_path, new_keeper)?;
        
        let mut permissions = fs::metadata(&full_path)?.permissions();
        permissions.set_mode(0o444); // Changed to read permission for all users
        fs::set_permissions(&full_path, permissions)?;
        
        println!("credentials::{}:{}", user, password);
    } else {
        println!("Credentials already exist");
    }

    Ok(())
}
