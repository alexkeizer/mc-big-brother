use std::net::SocketAddr;

use crate::db::models::computer::*;
use crate::server::*;

mod db;
mod server;
mod response;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log4rs::init_file("logging.yaml", Default::default())?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 7777));

    let server = Box::leak(Box::new(Server::new().await?));

    server.run_tcp(addr).await?;
    Ok(())
}
