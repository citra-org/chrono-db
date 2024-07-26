use chronodb::{managers, server};
use ini::Ini;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let conf =
        Ini::load_from_file("conf.ini").map_err(|e| format!("Failed to load INI file: {}", e))?;

    let section = conf
        .section(Some("CHRONO_DB"))
        .ok_or("Section 'CHRONO_DB' not found in the INI file")?;

    let chrono_uri = section
        .get("CHRONO_URI")
        .ok_or("CHRONO_URI not found in 'CHRONO_DB' section")?;

    let conn_info = managers::parser::uri::parse_itlg_uri(chrono_uri)
        .map_err(|e| format!("Failed to parse CHRONO_URI: {}", e))?;

    println!("Starting server on {}:{}", conn_info.host, conn_info.port);

    server::tcp::run_server(
        &conn_info.host,
        conn_info.port,
        &conn_info.chrono,
        &conn_info.keeper,
        &conn_info.secret,
    )
    .map_err(|e| format!("Failed to run server: {}", e))?;

    Ok(())
}