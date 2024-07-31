use crate::assist;
use crate::managers;
use num_cpus;
use std::fs::OpenOptions;
use std::io::{BufWriter, Error, ErrorKind, Write};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

pub fn write_events(
    chrono: &str,
    stream: &str,
    events: Vec<(String, String)>,
) -> Result<(), Error> {
    if let Err(e) = managers::folders::check::check_folder(chrono, true) {
        eprintln!("Failed to check folder: {}", e);
        return Err(Error::new(ErrorKind::Other, "Failed to check folder"));
    }

    let start_time = Instant::now();
    let events = Arc::new(events);
    let file_path = format!("/var/lib/citra/chrono/{}/{}.chrono", chrono, stream).to_string();
    let file_path = Arc::new(file_path);
    if let Err(e) = managers::f3iles::check::check_file(&file_path, true) {
        eprintln!("Failed to check file: {}", e);
        return Err(Error::new(ErrorKind::Other, "Failed to check file"));
    }

    let num_cpus = num_cpus::get();
    let events_per_thread = (events.len() + num_cpus - 1) / num_cpus;

    let mut handles = vec![];
    for i in 0..num_cpus {
        let file_path = file_path.clone();
        let events = events.clone();
        let handle = thread::spawn(move || -> Result<(), Error> {
            let start = i * events_per_thread;
            let end = std::cmp::min((i + 1) * events_per_thread, events.len());

            if start >= events.len() {
                return Ok(());
            }

            let file = match OpenOptions::new().append(true).open(&*file_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to open file: {:?}", e);
                    return Err(e);
                }
            };
            let mut buf_writer = BufWriter::with_capacity(8192, file);
            for (tag, entry) in events[start..end].iter() {
                let stamp: u128 = assist::time::get_current_time(true);
                if let Err(e) =
                    buf_writer.write_all(format!("{} {} {}\n", stamp, tag, entry).as_bytes())
                {
                    eprintln!("Failed to write to buffer: {:?}", e);
                    return Err(e);
                }
            }
            if let Err(e) = buf_writer.flush() {
                eprintln!("Failed to flush buffer: {:?}", e);
                return Err(e);
            }
            Ok(())
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join().unwrap() {
            return Err(e);
        }
    }

    let duration = start_time.elapsed();
    println!("CPU's: {}", num_cpus);
    println!("Time taken: {:?}", duration);
    println!("Events written: {}", events.len());
    println!(
        "Write speed: {} events/second",
        events.len() as f64 / duration.as_secs_f64()
    );
    Ok(())
}
