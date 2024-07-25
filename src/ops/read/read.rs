use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_events(chrono: &str, stream: &str) -> Result<String, Error> {
    let file = File::open(format!("{}/{}.itlg", chrono, stream).to_string())?;
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
