use crate::prelude::*;
/// Bits 1-6: Represent FROM position as index 0 to 63
/// Bits 7-12: Represent TO position as index 0 to 63
/// Bits 13-16: Represent this:
/// Source: https://www.chessprogramming.org/Encoding_Moves
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[rustfmt::skip]
pub enum MoveType {
    /// pawn 2 forward
    DoubleMove =        0b0001_0000_0000_0000, // 1
    KingCastle =        0b0010_0000_0000_0000, // 2
    QueenCastle =       0b0011_0000_0000_0000, // 3
    Capture =           0b0100_0000_0000_0000, // 4
    // en passant
    EpCapture =         0b0101_0000_0000_0000, // 5

    KnightPromo =       0b1000_0000_0000_0000, // 8
    BishopPromo =       0b1001_0000_0000_0000, // 9
    RookPromo =         0b1010_0000_0000_0000, // 10
    QueenPromo =        0b1011_0000_0000_0000, // 11
    KnightPromoCapture= 0b1100_0000_0000_0000, // 12
    BishopPromoCapture= 0b1101_0000_0000_0000, // 13
    RookPromoCapture =  0b1110_0000_0000_0000, // 14
    QueenPromoCapture = 0b1111_0000_0000_0000, // 15
    /// everything else
    Quiet =             0b0000_0000_0000_0000, // 0
}

impl MoveType {
    pub const fn to_promotion_piece(&self) -> Option<Piece> {
        match self {
            MoveType::KnightPromo | MoveType::KnightPromoCapture => Some(Knight),
            MoveType::BishopPromo | MoveType::BishopPromoCapture => Some(Bishop),
            MoveType::RookPromo | MoveType::RookPromoCapture => Some(Rook),
            MoveType::QueenPromo | MoveType::QueenPromoCapture => Some(Queen),
            _ => None,
        }
    }

    pub const fn to_promotion_color_piece(&self, color: Color) -> Option<Figure> {
        match (self, color) {
            (MoveType::KnightPromo | MoveType::KnightPromoCapture, White) => {
                Some(Figure::WhiteKnight)
            }
            (MoveType::BishopPromo | MoveType::BishopPromoCapture, White) => {
                Some(Figure::WhiteBishop)
            }
            (MoveType::RookPromo | MoveType::RookPromoCapture, White) => Some(Figure::WhiteRook),
            (MoveType::QueenPromo | MoveType::QueenPromoCapture, White) => Some(Figure::WhiteQueen),
            (MoveType::KnightPromo | MoveType::KnightPromoCapture, Black) => {
                Some(Figure::BlackKnight)
            }
            (MoveType::BishopPromo | MoveType::BishopPromoCapture, Black) => {
                Some(Figure::BlackBishop)
            }
            (MoveType::RookPromo | MoveType::RookPromoCapture, Black) => Some(Figure::BlackRook),
            (MoveType::QueenPromo | MoveType::QueenPromoCapture, Black) => Some(Figure::BlackQueen),
            _ => None,
        }
    }

    pub const fn is_promotion(&self) -> bool {
        (*self as u16) & 0b1000_0000_0000_0000 != 0
    }

    pub const fn from_u16(value: u16) -> Self {
        match value {
            0b0000000000000000 => MoveType::Quiet,
            0b0001000000000000 => MoveType::DoubleMove,
            0b0010000000000000 => MoveType::KingCastle,
            0b0011000000000000 => MoveType::QueenCastle,
            0b0100000000000000 => MoveType::Capture,
            0b0101000000000000 => MoveType::EpCapture,
            // 6, 7 are unused
            0b1000000000000000 => MoveType::KnightPromo,
            0b1001000000000000 => MoveType::BishopPromo,
            0b1010000000000000 => MoveType::RookPromo,
            0b1011000000000000 => MoveType::QueenPromo,
            0b1100000000000000 => MoveType::KnightPromoCapture,
            0b1101000000000000 => MoveType::BishopPromoCapture,
            0b1110000000000000 => MoveType::RookPromoCapture,
            0b1111000000000000 => MoveType::QueenPromoCapture,
            _ => panic!("could not convert u16 to move type"),
        }
    }
}
