use crate::assist;
use std::fs::{self};
use std::io::Result;
use std::os::unix::fs::PermissionsExt;

const DEFAULT_KEEPER: &str = "admin";
const CONFIG_FILE_PATH: &str = "~/.itlg/config";

pub fn create_keeper(keeper: Option<&str>) -> Result<()> {
    let config_path = assist::path::normalize_path(CONFIG_FILE_PATH);

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = match fs::read_to_string(&config_path) {
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

        fs::write(&config_path, new_keeper)?;

        let mut permissions = fs::metadata(&config_path)?.permissions();
        permissions.set_mode(0o400);
        fs::set_permissions(&config_path, permissions)?;

        println!("New credentials created");
    } else {
        println!("Credentials already exist");
    }

    Ok(())
}
