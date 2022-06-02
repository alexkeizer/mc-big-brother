use std::error::Error;
use std::net::SocketAddr;

use log::info;

use crate::computer::Computer;
use crate::server::Server;

mod computer;
mod db;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("logging.yaml", Default::default())?;

    info!("Marking all computers as offline");
    Computer::set_online_all(false).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 7777));

    Server::run(addr).await
}
