use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct EncodedMove(pub u16);

impl EncodedMove {
    pub const fn decode(self) -> DecodedMove {
        let from = Square((self.0 & 0b0000000000111111) as usize);
        let to = Square(((self.0 & 0b0000111111000000) >> 6) as usize);
        let mv_type = MoveType::from_u16(self.0 & 0b1111000000000000);
        DecodedMove { from, to, mv_type }
    }

    pub const fn encode(from: Square, to: Square, mv_type: MoveType) -> EncodedMove {
        let from_idx = from.0 as u16;
        let to_idx = to.0 as u16;
        EncodedMove(from_idx as u16 | (to_idx) << 6 | (mv_type as u16))
    }
}
