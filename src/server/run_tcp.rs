use std::convert::Infallible;
use std::error::Error;
use std::time::Duration;

use log::{info, warn};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use crate::Computer;

use super::Server;

const MAGIC: u32 = 0x6B7109BA;

impl Server {
    async fn handle_connection(&self, mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
        info!("Handling new connection");

        let magic = socket.read_u32().await?;
        if magic != MAGIC {
            warn!("Invalid magic bytes. Expected {MAGIC:x}, found {magic:x}");
            return Ok(())
        }

        let computer = Computer::from_socket(&mut socket).await?;

        info!("New connection from {:?}", computer);

        {
            self.online_computers.lock()
                .await
                .insert(computer);
        }

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
                    }
                }
            };

            let repeat = (std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis() % 4) as usize;
            info!("Sending {repeat} no-op Pong(s) to {computer:?}");
            let send_buf = [0u8; 1];
            for _ in 0..repeat {
                socket.write_all(&send_buf).await?;
            }
            socket.flush().await?;
        }


        self.online_computers.lock()
            .await
            .remove(&computer);
        Ok(())
    }

    pub async fn run_tcp(&'static self, addr: impl ToSocketAddrs) -> Result<Infallible, Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;
        info!("Bound socket to {}", listener.local_addr()?);

        loop {
            let (socket, _) = listener.accept().await?;

            tokio::spawn(
                async {
                    match self.handle_connection(socket).await {
                        Ok(_) => {}
                        Err(e) => {
                            warn!("{}", e)
                        }
                    };
                }
            );
        }
    }
}