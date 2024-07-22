use intellilog_database::managers;
use intellilog_database::server;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let connection_string = "itlg://admin:z1yehwtqlm1T7oPr@127.0.0.1:3141/database";
    let conn_info = managers::parser::uri::parse_itlg_uri(connection_string)?;
    //TODO: remove this url & handle it in another service
    println!("Starting server on {}:{}", conn_info.host, conn_info.port);
    server::tcp::run_server( &conn_info.host, conn_info.port, &conn_info.chrono,&conn_info.keeper, &conn_info.secret)

}