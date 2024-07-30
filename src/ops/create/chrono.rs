use crate::managers;
use std::io::Result;

pub fn create_chrono(keeper: Option<&str>) -> Result<()> {
    let name = keeper.unwrap_or("default");
    let base_path = "/var/lib/citra/chrono";

    let full_path = format!("{}/{}", base_path, name);

    match managers::folders::create::create_folder(&full_path) {
        Ok(_) => println!("Chrono '{}' created or exists at {:?}", name, full_path),
        Err(e) => eprintln!("Error creating Chrono '{}': {}", name, e),
    }

    Ok(())
}
