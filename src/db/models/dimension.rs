
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct DimensionId(pub i16);

#[allow(dead_code)]
impl DimensionId {
    const OVERWORLD: i16 = 0;

    #[inline]
    pub fn inner(&self) -> i16 {
        self.0
    }
}