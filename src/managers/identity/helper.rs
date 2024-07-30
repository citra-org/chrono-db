use crate::assist;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn validate_credentials_helper(
    chrono: &str,
    keeper: &str,
    secret: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let config_file_path: &str = &format!("~/.citra/{}/chrono_config", chrono);
    let config_path = assist::path::normalize_path(config_file_path);
    let file = File::open(config_path)?;

    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 && parts[0] == keeper && parts[1] == secret {
            return Ok(true);
        }
    }

    Ok(false)
}
