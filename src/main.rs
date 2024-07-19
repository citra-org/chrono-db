use std::env;

mod ops {
    pub mod read;
    pub mod write;
    pub mod create;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let file_name = &format!("{}.itlg", args[1]);

    match args[2].as_str() {
        "c" => ops::create::create_record(file_name)?,
        "w" => ops::write::write_record(file_name, args[3..].chunks(2).map(|chunk| (chunk[0].clone(), chunk[1].clone())).collect::<Vec<_>>())?,
        "r" => ops::read::read_records(file_name)?,
        _ => {
            eprintln!("Unknown command: {}", args[2]);
        }
    }

    Ok(())
}