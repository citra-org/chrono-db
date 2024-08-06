use chronodb::server;
use chronodb::jobs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    server::initializer::initializer()?;
    jobs::initializer::initializer()?;
    Ok(())
}
