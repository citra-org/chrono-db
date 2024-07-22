use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_events(stream: &str) -> Result<String, Error> {
    let file = File::open(stream)?;
    let streamer = BufReader::new(file);
    let mut events = String::new();
    for line in streamer.lines() {
        let line = line?;
        let mut parts = line.splitn(3, ' ');
        if let (Some(stamp), Some(tag), Some(entry)) = (parts.next(), parts.next(), parts.next()) {
            events.push_str(&format!("{} {} {}\n", stamp, tag, entry));
            //TODO: fix this to send lines instead of lines as string
        } else {
            events.push_str(&format!("Malformed event: {}\n", line));
        }
    }
    println!("hgehehhehe {}", events);
    Ok(events)
}
