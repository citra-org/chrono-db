use ini::Ini;
use rand::Rng;
use std::fs::{self, OpenOptions};
use std::io::Seek;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::time::SystemTime;
enum DeletionCriteria {
    TimeBased(u64),
    StorageBased(u64),
}

pub fn secure_delete_logs() -> std::io::Result<()> {
    let conf = Ini::load_from_file("conf.ini").expect("Failed to load config file");
    let section = conf.section(None::<String>).expect("Failed to get section");

    let method = section.get("METHOD").expect("Failed to get METHOD");
    let value: u64 = section
        .get("VALUE")
        .expect("Failed to get VALUE")
        .parse()
        .expect("Failed to parse VALUE");

    let criteria = match method {
        "TIME" => DeletionCriteria::TimeBased(value),
        "STORAGE" => DeletionCriteria::StorageBased(value),
        _ => panic!("Invalid METHOD in config file"),
    };

    let log_directory = Path::new("/var/lib/citra/chrono");

    let mut log_files: Vec<(PathBuf, SystemTime, u64)> = Vec::new();

    for chrono_entry in fs::read_dir(log_directory)? {
        let chrono_entry = chrono_entry?;
        let chrono_path = chrono_entry.path();
        if chrono_path.is_dir() {
            for stream_entry in fs::read_dir(&chrono_path)? {
                let stream_entry = stream_entry?;
                let path = stream_entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("chrono") {
                    let metadata = fs::metadata(&path)?;
                    let modified_time = metadata.modified()?;
                    let file_size = metadata.len();
                    log_files.push((path, modified_time, file_size));
                }
            }
        }
    }

    match criteria {
        DeletionCriteria::TimeBased(days) => {
            let cutoff = SystemTime::now() - Duration::from_secs(days * 24 * 60 * 60);
            for (path, modified_time, _) in log_files {
                if modified_time < cutoff {
                    secure_delete_file(&path)?;
                } else {
                    break;
                }
            }
        }
        DeletionCriteria::StorageBased(gb_limit) => {
            let byte_limit = gb_limit * 1024 * 1024 * 1024;
            let mut total_size: u64 = log_files.iter().map(|&(_, _, size)| size).sum();

            for (path, _, file_size) in log_files {
                if total_size <= byte_limit {
                    break;
                }
                secure_delete_file(&path)?;
                total_size -= file_size;
            }
        }
    }

    Ok(())
}

fn secure_delete_file(path: &Path) -> std::io::Result<()> {
    let file_size = fs::metadata(path)?.len();
    let mut file = OpenOptions::new().write(true).open(path)?;
    let mut rng = rand::thread_rng();

    for _ in 0..3 {
        file.set_len(0)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        for _ in 0..file_size {
            let random_byte: u8 = rng.gen();
            file.write_all(&[random_byte])?;
        }
        file.sync_all()?;
    }

    fs::remove_file(path)?;

    Ok(())
}
