use std::convert::Infallible;
use std::time::Duration;

use log::{info, warn};
use tokio::io::{AsyncReadExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use crate::*;


const MAGIC: u32 = 0x6B7109BA;

/// Reads the initial header sent by clients when establishing new connections
/// Expected format (all ints in big-endian order):
///     4 bytes                 magic value (must always be 0x6B7109BA)
///     4 bytes unsigned int    client version
///     2 bytes unsigned int    world     (i.e., minecraft server)
///     2 bytes signed int      dimension (e.g., 0 for Overworld, 1 for End, -1 for Nether)
///     4 bytes signed int      x coordinate
///     4 bytes signed int      z coordinate
async fn read_connection_header(socket: &mut TcpStream) -> anyhow::Result<ComputerData> {
    let magic = socket.read_u32().await?;
    if magic != MAGIC {
        anyhow::bail!("Invalid magic bytes. Expected {MAGIC:x}, found {magic:x}");
    }

    Ok(ComputerData {
        version: socket.read_u32().await?,
        world: WorldId(socket.read_u16().await?),
        dimension: DimensionId(socket.read_i16().await?),
        pos_x: socket.read_i32().await?,
        pos_z: socket.read_i32().await?,
    })
}

impl Server {
    async fn handle_connection(&self, mut socket: TcpStream) -> anyhow::Result<()> {
        info!("Handling new connection");

        let data = read_connection_header(&mut socket).await?;
        let computer = self.computer_repo.upsert(data).await?;

        info!("New connection from {:?}", computer);

        // In a loop, read data from the socket and write the data back.
        loop {
            let mut buf = [0; 128];
            let read_fut = tokio::time::timeout(
                Duration::from_secs(5),
                socket.read(&mut buf),
            ).await;

            match read_fut {
                // timeout elapsed
                Err(_) => {
                    info!("Timeout elapsed, polling: {:?}", computer);
                    break;
                }

                Ok(r) => match r {
                    // socket closed
                    Ok(n) if n == 0 => {
                        warn!("socket closed by client {:?}", computer);
                        break;
                    }

                    Err(e) => {
                        warn!("failed to read from socket ({:?}); err = {:?}", computer, e);
                        break;
                    }
                    Ok(_) => {
                        info!("POLL from {:?}", computer);
                        self.computer_repo.insert_ping(&computer).await?;
                    }
                }
            };
        }

        Ok(())
    }

    pub async fn run_tcp(&'static self, addr: impl ToSocketAddrs) -> anyhow::Result<Infallible> {
        let listener = TcpListener::bind(addr).await?;
        info!("Bound socket to {}", listener.local_addr()?);

        loop {
            let (socket, _) = listener.accept().await?;

            tokio::spawn(
                async {
                    match self.handle_connection(socket).await {
                        Ok(_) => {}
                        Err(e) => {
                            warn!("Connection died unexpectedly: {}", e)
                        }
                    };
                }
            );
        }
    }
}