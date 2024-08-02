use crate::managers;
use std::io::Result;

pub fn delete_stream(chrono: &str, stream: &str) -> Result<()> {
    //TODO: update this code as per new path
    match managers::files::delete::delete_file(chrono, &(stream.to_string() + ".chrono")) {
        Ok(_) => println!("stream '{}' deleted or doesnt exists", stream),
        Err(e) => eprintln!("error deleting stream '{}': {}", stream, e),
    }

    Ok(())
}
