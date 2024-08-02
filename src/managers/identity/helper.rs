use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn validate_credentials_helper(
    chrono: &str,
    keeper: &str,
    secret: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let config_file_path: &str = &format!("/home/kali/.citra/chrono/{}/config", chrono);
    let file = File::open(config_file_path)?;

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
