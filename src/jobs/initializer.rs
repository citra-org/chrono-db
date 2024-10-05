use crate::jobs::purge;
use std::io::Result;
use std::thread;
use std::time::Duration;

pub fn initializer() -> Result<()> {
    //TODO: Testing for purge job (delete logs automatically)
    if let Err(e) = purge::secure_delete_logs() {
        eprintln!("Error during initial log deletion: {}", e);
    }

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        if let Err(e) = purge::secure_delete_logs() {
            eprintln!("Error during periodic log deletion: {}", e);
        }
    });

    Ok(())
}
