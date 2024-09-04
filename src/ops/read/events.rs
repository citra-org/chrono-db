use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::sync::Arc;

pub fn read_events(chrono: &str, stream: &str) -> Result<String, Error> {
    let file_path = format!("/var/lib/citra/chrono/{}/{}.chrono", chrono, stream).to_string();
    let file_path = Arc::new(file_path);

    let file = File::open(file_path.to_string())?;

    let streamer = BufReader::new(file);
    let mut events = String::new();
    for line in streamer.lines() {
        let line = line?;
        let mut parts = line.splitn(3, ' ');
        if let (Some(stamp), Some(tag), Some(entry)) = (parts.next(), parts.next(), parts.next()) {
            events.push_str(&format!("{} {} {}\n", stamp, tag, entry));
        } else {
            events.push_str(&format!("Malformed event: {}", line));
        }
    }
    Ok(events)
}
