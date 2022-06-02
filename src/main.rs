use std::error::Error;
use std::net::SocketAddr;

use crate::computer::*;
use crate::server::*;

mod computer;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("logging.yaml", Default::default())?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 7777));

    let server = Box::leak(Box::new(Server::new()));

    server.run_tcp(addr).await?;
    Ok(())
}
