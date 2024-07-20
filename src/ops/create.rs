use std::fs::File;
use std::io::Write;

pub fn create_record(file_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut file = File::create(file_name)?;
    file.write_all(b"New ITLG file\n")?;
    Ok(())
}
