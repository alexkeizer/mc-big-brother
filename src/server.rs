use std::error::Error;
use std::time::Duration;

use log::{info, warn};
use tokio::io::{AsyncReadExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use crate::Computer;

pub struct Server {}

impl Server {
    fn new() -> Self {
        Server {}
    }

    async fn handle_connection(&self, mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
        info!("Handling new connection");

        let computer = {
            let mut computer = Computer {
                id: socket.read_i64().await?,
                chunk_x: socket.read_i64().await?,
                chunk_y: socket.read_i64().await?,
            };

            info!("New connection from {:?}", computer);

            if computer.id == 0 {
                // also sets is_online to true
                computer.insert(true).await?;

                info!("inserted new record {:?}", computer);
            } else {
                computer.set_online(true).await?;

                info!("marked {:?} as online", computer);
            }

            computer
        };

        let cleanup = || {
            computer.set_online(false)
        };

        // In a loop, read data from the socket and write the data back.
        loop {
            let mut buf = [0; 128];
            let read_fut = tokio::time::timeout(
                Duration::from_secs(2),
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

            // let send_buf = [0; 256];
            //
            // // Try to write some data, to check the socket is still available
            // if let Err(e) = socket.write_all(&send_buf).await {
            //     warn!("failed to write to socket ({:?}); err = {:?}", computer, e);
            //     break;
            // }
        }
        cleanup().await?;
        Ok(())
    }

    pub async fn run(addr: impl ToSocketAddrs) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;
        info!("Bound socket to {}", listener.local_addr()?);

        let server_box = Box::new(Server::new());
        let server = Box::leak(server_box);

        loop {
            let (socket, _) = listener.accept().await?;

            tokio::spawn(
                async {
                    match server.handle_connection(socket).await {
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