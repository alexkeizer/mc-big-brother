// #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
// #[repr(u8)]
// #[non_exhaustive]
// pub enum Dimension {
//     Overworld = 0,
//     Nether = 1,
//     End = 2,
// }

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, sqlx::Encode)]
pub struct ComputerId(u32);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct ComputerData {
    pub version: u32,
    // The version of `open_socket` script this computer is running
    pub world: u16,
    // which (Minecraft) server
    pub dimension: u16,
    // the dimension (Overworld, End, Nether)
    pub pos_x: i32,
    pub pos_z: i32,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Computer {
    pub id: ComputerId,
    pub data: ComputerData,
}


impl ComputerId {
    pub(in crate::db) fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn inner(&self) -> u32 {
        self.0
    }
}

impl From<&Computer> for ComputerId {
    fn from(value: &Computer) -> Self {
        value.id
    }
}