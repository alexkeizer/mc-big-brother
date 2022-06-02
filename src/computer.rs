use std::error::Error;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;

// #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
// #[repr(u8)]
// #[non_exhaustive]
// pub enum Dimension {
//     Overworld,
//     Nether,
//     End,
// }


#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Computer {
    pub world: u16,     // which (Minecraft) server
    pub dimension: u8,  // the dimension (Overworld, End, Nether)
    pub chunk_x: i32,
    pub chunk_y: i32,
}

impl Computer {
    pub async fn from_socket(socket : &mut TcpStream) -> Result<Self, Box<dyn Error>> {

        Ok (Self {
            world : socket.read_u16().await?,
            dimension: socket.read_u8().await?,
            chunk_x: socket.read_i32().await?,
            chunk_y: socket.read_i32().await?
        })
    }
}