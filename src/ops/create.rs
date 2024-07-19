use std::fs::File;
use std::io::Result;

pub fn create_record(file_path: &str) -> Result<()> {
    File::create(file_path)?;
    Ok(())
}