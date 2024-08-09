use crate::assist;
use crate::managers;
use std::fs::OpenOptions;
use std::io::{BufWriter, Error, ErrorKind, Write};
use std::sync::{Arc, Mutex};
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
    let file_path = Arc::new(format!(
        "/var/lib/citra/chrono/{}/{}.chrono",
        chrono, stream
    ));
    // if let Err(e) = managers::files::check::check_file(&file_path, true) {
    //     eprintln!("Failed to check file: {}", e);
    //     return Err(Error::new(ErrorKind::Other, "Failed to check file"));
    // }
    let events_length = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for (header, body) in events.into_iter() {
        let file_path = Arc::clone(&file_path);
        let events_length = Arc::clone(&events_length);
        let handle = thread::spawn(move || {
            let file = match OpenOptions::new().append(true).open(&*file_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to open file: {:?}", e);
                    return;
                }
            };

            let mut buf_writer = BufWriter::with_capacity(8192, file);

            let stamp: u128 = assist::time::get_current_time(true);
            if let Err(e) =
                buf_writer.write_all(format!("{} {} {}\n", stamp, header, body).as_bytes())
            {
                eprintln!("Failed to write to buffer: {:?}", e);
                return;
            }

            if let Err(e) = buf_writer.flush() {
                eprintln!("Failed to flush buffer: {:?}", e);
                return;
            }

            let mut count = events_length.lock().unwrap();
            *count += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start_time.elapsed();
    let events_length = *events_length.lock().unwrap();

    // println!("Time taken: {:?}", duration);
    // println!("Events written: {}", events_length);
    // println!(
    //     "Written : {} events/second",
    //     (events_length as f64 / duration.as_secs_f64()) as i64
    // );

    Ok(())
}
