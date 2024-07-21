use std::fs::File;

pub fn create_record(file_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    File::create(file_name)?;
    Ok(())
}
