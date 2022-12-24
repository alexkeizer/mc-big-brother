
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct WorldId(pub u16);


#[allow(dead_code)]
impl WorldId {
    const TEST: u16 = 0;
    const VOLTZ_VOIDFLAME: u16 = 1;

    #[inline]
    pub fn inner(&self) -> u16 {
        self.0
    }
}