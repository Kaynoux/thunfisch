use crate::prelude::*;

// Bits 1-6: Represent FROM position as index 0 to 63
// Bits 7-12: Represent TO position as index 0 to 63
// Bits 13-16: Represent this:
// Source: https://www.chessprogramming.org/Encoding_Moves
#[rustfmt::skip]
mod unformatted {
    pub const QUIET:                u16 = 0b0000; // 0
    pub const DOUBLE_MOVE:     u16 = 0b0001; // 1
    pub const KING_CASTLE:          u16 = 0b0010; // 2
    pub const QUEEN_CASTLE:         u16 = 0b0011; // 3
    pub const CAPTURE:              u16 = 0b0100; // 4
    pub const EP_CAPTURE:           u16 = 0b0101; // 5
    // 6,7 unused
    pub const KNIGHT_PROMO:         u16 = 0b1000; // 8
    pub const BISHOP_PROMO:         u16 = 0b1001; // 9
    pub const ROOK_PROMO:           u16 = 0b1010; // 10
    pub const QUEEN_PROMO:          u16 = 0b1011; // 11
    pub const KNIGHT_PROMO_CAPTURE: u16 = 0b1100; // 12
    pub const BISHOP_PROMO_CAPTURE: u16 = 0b1101; // 13
    pub const ROOK_PROMO_CAPTURE:   u16 = 0b1110; // 14
    pub const QUEEN_PROMO_CAPTURE:  u16 = 0b1111; // 15
}
use unformatted::*;

#[derive(Copy, Clone, PartialEq)]
pub struct ChessMove(pub u16);

impl ChessMove {
    pub const fn encode(
        from: Position,
        to: Position,
        is_capture: bool,
        is_double_move: bool,
        is_ep_capture: bool,
        is_king_castle: bool,
        is_queen_castle: bool,
        promotion: Option<Piece>,
    ) -> ChessMove {
        let from_index = from.to_index() as u16;
        let to_index = to.to_index() as u16;

        let last_nibble: u16 = if let Some(piece) = promotion {
            if is_capture {
                match piece {
                    Piece::Queen => QUEEN_PROMO_CAPTURE,
                    Piece::Rook => ROOK_PROMO_CAPTURE,
                    Piece::Bishop => BISHOP_PROMO_CAPTURE,
                    Piece::Knight => KNIGHT_PROMO_CAPTURE,
                    _ => QUIET,
                }
            } else {
                match piece {
                    Piece::Queen => QUEEN_PROMO,
                    Piece::Rook => ROOK_PROMO,
                    Piece::Bishop => BISHOP_PROMO,
                    Piece::Knight => KNIGHT_PROMO,
                    _ => QUIET,
                }
            }
        } else if is_capture {
            CAPTURE
        } else if is_ep_capture {
            EP_CAPTURE
        } else if is_king_castle {
            KING_CASTLE
        } else if is_queen_castle {
            QUEEN_CASTLE
        } else if is_double_move {
            DOUBLE_MOVE
        } else {
            QUIET
        };

        ChessMove(from_index | to_index << 6 | last_nibble << 12)
    }
}
