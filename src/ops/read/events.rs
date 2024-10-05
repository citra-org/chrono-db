use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};

pub fn read_events(chrono: &str, stream: &str) -> Result<String, io::Error> {
    let file_path = format!("/var/lib/citra/chrono/{}/{}.chrono", chrono, stream);

    let file = File::open(&file_path).map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
    let mut file = BufReader::new(file);

    let file_len = file
        .seek(SeekFrom::End(0))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    const CHUNK_SIZE: usize = 1024;
    let mut buffer = Vec::new();
    let mut pos = file_len;

    while pos > 0 {
        let seek_pos = if pos > CHUNK_SIZE as u64 {
            pos - CHUNK_SIZE as u64
        } else {
            0
        };

        file.seek(SeekFrom::Start(seek_pos))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let mut chunk = vec![0; (pos - seek_pos) as usize];
        file.read_exact(&mut chunk)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        buffer.splice(0..0, chunk);

        pos = seek_pos;

        if buffer.iter().filter(|&&b| b == b'\n').count() >= 10 {
            break;
        }
    }

    let content =
        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let lines: Vec<&str> = content.lines().rev().take(10).collect();

    let mut events = String::new();
    for line in lines.iter().rev() {
        let mut parts = line.splitn(3, ' ');
        if let (Some(stamp), Some(tag), Some(entry)) = (parts.next(), parts.next(), parts.next()) {
            events.push_str(&format!("{} {} {}\n", stamp, tag, entry));
        } else {
            events.push_str(&format!("Malformed event: {}\n", line));
        }
    }

    Ok(events)
}
