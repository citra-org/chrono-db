use std::time::Duration;
use std::thread;
use std::io::Result;
use crate::jobs::purge;

pub fn initializer() -> Result<()> {
    //TODO: Testing for purge job (delete logs automatically)
    if let Err(e) = purge::secure_delete_logs() {
        eprintln!("Error during initial log deletion: {}", e);
    }

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1)); 
            if let Err(e) = purge::secure_delete_logs() {
                eprintln!("Error during periodic log deletion: {}", e);
            }
        }
    });

    Ok(())
}