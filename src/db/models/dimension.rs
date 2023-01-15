
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct DimensionId(pub i16);

#[allow(dead_code)]
impl DimensionId {
    const OVERWORLD: i16 = 0;
    const KEPLER22B: i16 = 1;
    #[inline]
    pub fn inner(&self) -> i16 {
        self.0
    }
}