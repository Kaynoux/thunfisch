use crate::prelude::*;

// Bits 1-6: Represent FROM position as index 0 to 63
// Bits 7-12: Represent TO position as index 0 to 63
// Bits 13-16: Represent this:
// Source: https://www.chessprogramming.org/Encoding_Moves
#[rustfmt::skip]
pub mod move_flags {
    pub const QUIET:                u16 = 0b0000000000000000; // 0
    pub const DOUBLE_MOVE:          u16 = 0b0000000000000001; // 1
    pub const KING_CASTLE:          u16 = 0b0000000000000010; // 2
    pub const QUEEN_CASTLE:         u16 = 0b0000000000000011; // 3
    pub const CAPTURE:              u16 = 0b0000000000000100; // 4
    pub const EP_CAPTURE:           u16 = 0b0000000000000101; // 5
    // 6,7 unused
    pub const KNIGHT_PROMO:         u16 = 0b0000000000001000; // 8
    pub const BISHOP_PROMO:         u16 = 0b0000000000001001; // 9
    pub const ROOK_PROMO:           u16 = 0b0000000000001010; // 10
    pub const QUEEN_PROMO:          u16 = 0b0000000000001011; // 11
    pub const KNIGHT_PROMO_CAPTURE: u16 = 0b0000000000001100; // 12
    pub const BISHOP_PROMO_CAPTURE: u16 = 0b0000000000001101; // 13
    pub const ROOK_PROMO_CAPTURE:   u16 = 0b0000000000001110; // 14
    pub const QUEEN_PROMO_CAPTURE:  u16 = 0b0000000000001111; // 15
}
use move_flags::*;

#[derive(Copy, Clone, PartialEq)]
pub struct EncodedMove(pub u16);

impl EncodedMove {
    pub const fn encode(
        from: Position,
        to: Position,
        flag: u16
    ) -> EncodedMove {
        let from_idx = from.to_index().0 as u16;
        let to_idx = to.to_index().0 as u16;
        EncodedMove(from_idx as u16 | (to_idx) << 6 | flag)
    }

    pub const fn decode(self) -> DecodedMove {
        let from = IndexPosition((self.0 & 0b1111110000000000) as usize);
        let to = IndexPosition((self.0 & 0b0000001111110000) as usize);
        let (
            is_capture,
            is_double_move,
            is_ep_capture,
            is_king_castle,
            is_queen_castle,
            promotion
        ): (bool, bool, bool, bool, bool, Option<Piece>) = 
            match self.0  {
               QUIET => (false, false, false, false, false, None),
               DOUBLE_MOVE => (false, true, false, false, false, None),
               KING_CASTLE => (false, false, false, true, false, None),
               QUEEN_CASTLE => (false, false, false, false, true, None),
               CAPTURE => (true, false, false, false, false, None),
               EP_CAPTURE => (true, false, true, false, false, None),
               KNIGHT_PROMO => (false, false, false, false, false, Some(Piece::Knight)),
               BISHOP_PROMO => (false, false, false, false, false, Some(Piece::Bishop)),
               ROOK_PROMO => (false, false, false, false, false, Some(Piece::Rook)),
               QUEEN_PROMO => (false, false, false, false, false, Some(Piece::Queen)),
               KNIGHT_PROMO_CAPTURE => {(true, false, false, false, false, Some(Piece::Knight))},
               BISHOP_PROMO_CAPTURE => (true, false, false, false, false, Some(Piece::Bishop)),
               ROOK_PROMO_CAPTURE => (true, false, false, false, false, Some(Piece::Rook)),
               QUEEN_PROMO_CAPTURE => (true, false, false, false, false, Some(Piece::Queen)),
               _ => (false, false, false, false, false, None)
            };

        DecodedMove { from: from, to: to, is_capture: is_capture, is_double_move: is_double_move, is_ep_capture: is_ep_capture, is_king_castle: is_king_castle, is_queen_castle: is_queen_castle, promotion: promotion }
                
    }
}
