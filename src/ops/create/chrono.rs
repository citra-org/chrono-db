use crate::managers;
use std::io::Result;

pub fn create_chrono(keeper: Option<&str>) -> Result<()> {
    let name = keeper.unwrap_or("default");
    
    match managers::folders::create::create_folder(name, false){
        Ok(_) => println!("chrono '{}' created or exists", name),
        Err(e) => eprintln!("error creating chrono '{}': {}", name, e),
    }
    
    Ok(())
}
