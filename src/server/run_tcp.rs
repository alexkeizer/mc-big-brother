use std::error::Error;
use std::time::Duration;

use log::{info, warn};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use crate::Computer;

use super::Server;

impl Server {
    async fn handle_connection(&self, mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
        info!("Handling new connection");


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


        self.online_computers.lock()
            .await
            .remove(&computer);
        Ok(())
    }

    pub async fn run_tcp(&'static self, addr: impl ToSocketAddrs) -> Result<(), Box<dyn Error>> {
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