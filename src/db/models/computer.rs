use crate::db::models::dimension::DimensionId;
use crate::db::models::world::WorldId;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct ComputerId(u32);


#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct ComputerData {
    /// The version of `open_socket` script this computer is running
    pub version: u32,
    /// which (Minecraft) server
    pub world: WorldId,
    /// the dimension (Overworld, End, Nether)
    pub dimension: DimensionId,
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