use std::fs::File;
use std::io::{self, BufRead};

pub fn read_records(file_path: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut result = String::new();
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.splitn(3, ' ');
        if let (Some(time_str), Some(body), Some(headers)) = (parts.next(), parts.next(), parts.next()) {
            result.push_str(&format!("{} {} {}&/n", time_str, body, headers));
            //TODO: fix this to send lines insted of lines as string
        } else {
            result.push_str(&format!("Malformed record: {}\n", line));
        }
    }
    println!("hgehehhehe {}",result);
    Ok(result)
}