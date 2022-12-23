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
    pub version: u32,       // The version of `open_socket` script this computer is running
    pub world: u16,         // which (Minecraft) server
    pub dimension: u16,     // the dimension (Overworld, End, Nether)
    pub pos_x: i32,
    pub pos_z: i32,
}

impl Computer {
    pub async fn from_socket(socket : &mut TcpStream) -> Result<Self, Box<dyn Error>> {

        Ok (Self {
            version: socket.read_u32().await?,
            world : socket.read_u16().await?,
            dimension: socket.read_u16().await?,
            pos_x: socket.read_i32().await?,
            pos_z: socket.read_i32().await?
        })
    }
}