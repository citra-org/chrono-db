mod ops;
mod server;
mod connection;

use connection::parse_itlg_url;


fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let connection_string = "itlg://admin:z1yehwtqlm1T7oPr@127.0.0.1:3141/database";
    let conn_info = parse_itlg_url(connection_string)?;
    
    println!("Starting server on {}:{}", conn_info.host, conn_info.port);
    server::run_server(&conn_info.host, conn_info.port)

}