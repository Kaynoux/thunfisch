use crate::prelude::*;

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Figure {
    WhitePawn = 0,
    BlackPawn = 1,
    WhiteKnight = 2,
    BlackKnight = 3,
    WhiteBishop = 4,
    BlackBishop = 5,
    WhiteRook = 6,
    BlackRook = 7,
    WhiteQueen = 8,
    BlackQueen = 9,
    WhiteKing = 10,
    BlackKing = 11,
    Empty = 12,
}

impl Figure {
    pub const fn to_polyglot(self) -> usize {
        match self {
            Self::BlackPawn => 0,
            Self::WhitePawn => 1,
            Self::BlackKnight => 2,
            Self::WhiteKnight => 3,
            Self::BlackBishop => 4,
            Self::WhiteBishop => 5,
            Self::BlackRook => 6,
            Self::WhiteRook => 7,
            Self::BlackQueen => 8,
            Self::WhiteQueen => 9,
            Self::BlackKing => 10,
            Self::WhiteKing => 11,
            Self::Empty => usize::MAX,
        }
    }

    pub const fn from_idx(idx: usize) -> Self {
        match idx {
            0 => Self::WhitePawn,
            1 => Self::BlackPawn,
            2 => Self::WhiteKnight,
            3 => Self::BlackKnight,
            4 => Self::WhiteBishop,
            5 => Self::BlackBishop,
            6 => Self::WhiteRook,
            7 => Self::BlackRook,
            8 => Self::WhiteQueen,
            9 => Self::BlackQueen,
            10 => Self::WhiteKing,
            11 => Self::BlackKing,
            _ => Self::Empty,
        }
    }

    pub const fn piece_and_color(self) -> (Piece, Color) {
        match self {
            Self::WhitePawn => (Pawn, White),
            Self::BlackPawn => (Pawn, Black),
            Self::WhiteKnight => (Knight, White),
            Self::BlackKnight => (Knight, Black),
            Self::WhiteBishop => (Bishop, White),
            Self::BlackBishop => (Bishop, Black),
            Self::WhiteRook => (Rook, White),
            Self::BlackRook => (Rook, Black),
            Self::WhiteQueen => (Queen, White),
            Self::BlackQueen => (Queen, Black),
            Self::WhiteKing => (King, White),
            Self::BlackKing => (King, Black),
            Self::Empty => (Empty, White), // Color does not matter
        }
    }

    pub const fn piece(self) -> Piece {
        match self {
            Self::WhitePawn | Self::BlackPawn => Pawn,
            Self::WhiteKnight | Self::BlackKnight => Knight,
            Self::WhiteBishop | Self::BlackBishop => Bishop,
            Self::WhiteRook | Self::BlackRook => Rook,
            Self::WhiteQueen | Self::BlackQueen => Queen,
            Self::WhiteKing | Self::BlackKing => King,
            Self::Empty => Empty,
        }
    }

    pub const fn from_piece_and_color(piece: Piece, color: Color) -> Self {
        match (piece, color) {
            (Pawn, White) => Self::WhitePawn,
            (Pawn, Black) => Self::BlackPawn,
            (Knight, White) => Self::WhiteKnight,
            (Knight, Black) => Self::BlackKnight,
            (Bishop, White) => Self::WhiteBishop,
            (Bishop, Black) => Self::BlackBishop,
            (Rook, White) => Self::WhiteRook,
            (Rook, Black) => Self::BlackRook,
            (Queen, White) => Self::WhiteQueen,
            (Queen, Black) => Self::BlackQueen,
            (King, White) => Self::WhiteKing,
            (King, Black) => Self::BlackKing,
            (Empty, _) => Self::Empty,
        }
    }
}
