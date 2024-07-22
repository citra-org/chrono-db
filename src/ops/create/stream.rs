use crate::managers;
use std::io::Result;

pub fn create_stream(keeper: Option<&str>) -> Result<()> {
    let name = keeper.unwrap_or("default");
    
    match managers::files::create::create_file("data/default",name, false){
        Ok(_) => println!("chrono '{}' created or exists", name),
        Err(e) => eprintln!("error creating chrono '{}': {}", name, e),
    }
    
    Ok(())
}
