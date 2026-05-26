use crate::prelude::*;
/// Bits 1-6: Represent FROM position as index 0 to 63
/// Bits 7-12: Represent TO position as index 0 to 63
/// Bits 13-16: Represent this:
/// Source: <https://www.chessprogramming.org/Encoding_Moves>
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
    pub const fn to_promotion_piece(self) -> Option<Piece> {
        match self {
            Self::KnightPromo | Self::KnightPromoCapture => Some(Knight),
            Self::BishopPromo | Self::BishopPromoCapture => Some(Bishop),
            Self::RookPromo | Self::RookPromoCapture => Some(Rook),
            Self::QueenPromo | Self::QueenPromoCapture => Some(Queen),
            _ => None,
        }
    }

    pub const fn to_promotion_color_piece(self, color: Color) -> Option<Figure> {
        match (self, color) {
            (Self::KnightPromo | Self::KnightPromoCapture, White) => Some(Figure::WhiteKnight),
            (Self::BishopPromo | Self::BishopPromoCapture, White) => Some(Figure::WhiteBishop),
            (Self::RookPromo | Self::RookPromoCapture, White) => Some(Figure::WhiteRook),
            (Self::QueenPromo | Self::QueenPromoCapture, White) => Some(Figure::WhiteQueen),
            (Self::KnightPromo | Self::KnightPromoCapture, Black) => Some(Figure::BlackKnight),
            (Self::BishopPromo | Self::BishopPromoCapture, Black) => Some(Figure::BlackBishop),
            (Self::RookPromo | Self::RookPromoCapture, Black) => Some(Figure::BlackRook),
            (Self::QueenPromo | Self::QueenPromoCapture, Black) => Some(Figure::BlackQueen),
            _ => None,
        }
    }

    pub const fn is_promotion(self) -> bool {
        (self as u16) & 0b1000_0000_0000_0000 != 0
    }

    pub const fn is_capture(self) -> bool {
        (self as u16) & 0b0100_0000_0000_0000 != 0
    }

    pub const fn from_u16(value: u16) -> Self {
        match value {
            0b0000_0000_0000_0000 => Self::Quiet,
            0b0001_0000_0000_0000 => Self::DoubleMove,
            0b0010_0000_0000_0000 => Self::KingCastle,
            0b0011_0000_0000_0000 => Self::QueenCastle,
            0b0100_0000_0000_0000 => Self::Capture,
            0b0101_0000_0000_0000 => Self::EpCapture,
            // 6, 7 are unused
            0b1000_0000_0000_0000 => Self::KnightPromo,
            0b1001_0000_0000_0000 => Self::BishopPromo,
            0b1010_0000_0000_0000 => Self::RookPromo,
            0b1011_0000_0000_0000 => Self::QueenPromo,
            0b1100_0000_0000_0000 => Self::KnightPromoCapture,
            0b1101_0000_0000_0000 => Self::BishopPromoCapture,
            0b1110_0000_0000_0000 => Self::RookPromoCapture,
            0b1111_0000_0000_0000 => Self::QueenPromoCapture,
            _ => panic!("could not convert u16 to move type"),
        }
    }
}
