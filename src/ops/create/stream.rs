use crate::managers;
use std::io::Result;

pub fn create_stream(chrono: &str, stream: &str) -> Result<()> {
    match managers::files::create::create_file(chrono, &(stream.to_string() + ".itlg"), false) {
        Ok(_) => println!("stream '{}' created or exists", stream),
        Err(e) => eprintln!("error creating stream '{}': {}", stream, e),
    }

    Ok(())
}
