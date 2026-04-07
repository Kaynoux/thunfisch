use crate::prelude::*;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Hash, Eq)]
pub struct EncodedMove(pub u16);

/// contains all information about a move in a u16
/// Format: `[type | to | from]`
impl EncodedMove {
    pub const fn decode(self) -> DecodedMove {
        let from = Square((self.0 & 0b0000_0000_0011_1111) as usize);
        let to = Square(((self.0 & 0b0000_1111_1100_0000) >> 6) as usize);
        let mv_type = MoveType::from_u16(self.0 & 0b1111_0000_0000_0000);
        DecodedMove { from, to, mv_type }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub const fn encode(from: Square, to: Square, mv_type: MoveType) -> Self {
        let from_idx = from.0 as u16;
        let to_idx = to.0 as u16;
        Self(from_idx | (to_idx) << 6 | (mv_type as u16))
    }
}

impl std::fmt::Debug for EncodedMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decoded = self.decode();
        f.debug_struct("EncodedMove")
            .field("from", &decoded.from)
            .field("to", &decoded.to)
            .field("mv_type", &decoded.mv_type)
            .finish()
    }
}
