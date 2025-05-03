use crate::prelude::Position;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IndexPosition(pub usize);

impl IndexPosition {
    #[inline(always)]
    pub const fn to_position(self) -> Position {
        Position(1u64 << self.0)
    }
}
