use std::fs::File;
use std::io::Read;
pub fn read_key() -> Result<[u8; 32], Box<dyn std::error::Error>> {
    let mut file = File::open(".itlgs")?;
    let mut key = [0u8; 32];
    file.read_exact(&mut key)?;
    Ok(key)
}