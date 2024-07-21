use std::fs::{self};
use std::io::ErrorKind;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::error::Error;
use rand::Rng;

const DEFAULT_USER: &str = "admin";
const CONFIG_FILE_PATH: &str = "~/.itlg/config";

fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        let home = std::env::var("HOME").unwrap();
        let path = path.strip_prefix("~/").unwrap();
        Path::new(&home).join(path)
    } else {
        PathBuf::from(path)
    }
}

fn generate_random_password(length: usize) -> String {
    const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET.chars().nth(idx).unwrap()
        })
        .collect()
}

pub fn generate_user() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config_path = expand_tilde(CONFIG_FILE_PATH);

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = match fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(ref e) if e.kind() == ErrorKind::NotFound => String::new(),
        Err(e) => return Err(Box::new(e)),
    };

    if content.trim().is_empty() {
        let user = DEFAULT_USER;
        let password = generate_random_password(16);
        let new_content = format!("{}:{}", user, password);
        
        fs::write(&config_path, new_content)?;

        let mut permissions = fs::metadata(&config_path)?.permissions();
        permissions.set_mode(0o400); // read-only permission
        fs::set_permissions(&config_path, permissions)?;

        println!("New credentials created");
    } else {
        println!("Credentials already exist");
    }

    Ok(())
}