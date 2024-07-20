use std::fs;
use std::error::Error;
use rand::Rng;

const DEFAULT_USER: &str = "admin";
const CONFIG_FILE_PATH: &str = "config.txt";

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
    let user = DEFAULT_USER;
    let password = generate_random_password(12);
    let content = format!("username: {}\npassword: {}\n", user, password);
    
    fs::write(CONFIG_FILE_PATH, content).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    println!("Configuration file created with default user and generated password.");
    Ok(())
}