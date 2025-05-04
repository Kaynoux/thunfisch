use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct EncodedMove(pub u16);

impl EncodedMove {
    pub const fn decode(self) -> DecodedMove {
        let from = IndexPosition((self.0 & 0b0000000000111111) as usize);
        let to = IndexPosition(((self.0 & 0b0000111111000000) >> 6) as usize);
        let mv_type = MoveType::from_u16(self.0 & 0b1111000000000000);
        DecodedMove { from, to, mv_type }
    }
}
