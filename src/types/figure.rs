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
    pub const fn from_idx(idx: usize) -> Figure {
        match idx {
            0 => Figure::WhitePawn,
            1 => Figure::BlackPawn,
            2 => Figure::WhiteKnight,
            3 => Figure::BlackKnight,
            4 => Figure::WhiteBishop,
            5 => Figure::BlackBishop,
            6 => Figure::WhiteRook,
            7 => Figure::BlackRook,
            8 => Figure::WhiteQueen,
            9 => Figure::BlackQueen,
            10 => Figure::WhiteKing,
            11 => Figure::BlackKing,
            _ => Figure::Empty,
        }
    }

    pub fn piece_and_color(self) -> (Piece, Color) {
        match self {
            Figure::WhitePawn => (Pawn, White),
            Figure::BlackPawn => (Pawn, Black),
            Figure::WhiteKnight => (Knight, White),
            Figure::BlackKnight => (Knight, Black),
            Figure::WhiteBishop => (Bishop, White),
            Figure::BlackBishop => (Bishop, Black),
            Figure::WhiteRook => (Rook, White),
            Figure::BlackRook => (Rook, Black),
            Figure::WhiteQueen => (Queen, White),
            Figure::BlackQueen => (Queen, Black),
            Figure::WhiteKing => (King, White),
            Figure::BlackKing => (King, Black),
            Figure::Empty => (Empty, White), // Color does not matter
        }
    }

    pub fn piece(self) -> Piece {
        match self {
            Figure::WhitePawn | Figure::BlackPawn => Pawn,
            Figure::WhiteKnight | Figure::BlackKnight => Knight,
            Figure::WhiteBishop | Figure::BlackBishop => Bishop,
            Figure::WhiteRook | Figure::BlackRook => Rook,
            Figure::WhiteQueen | Figure::BlackQueen => Queen,
            Figure::WhiteKing | Figure::BlackKing => King,
            Figure::Empty => Empty,
        }
    }

    pub const fn from_piece_and_color(piece: Piece, color: Color) -> Figure {
        match (piece, color) {
            (Pawn, White) => Figure::WhitePawn,
            (Pawn, Black) => Figure::BlackPawn,
            (Knight, White) => Figure::WhiteKnight,
            (Knight, Black) => Figure::BlackKnight,
            (Bishop, White) => Figure::WhiteBishop,
            (Bishop, Black) => Figure::BlackBishop,
            (Rook, White) => Figure::WhiteRook,
            (Rook, Black) => Figure::BlackRook,
            (Queen, White) => Figure::WhiteQueen,
            (Queen, Black) => Figure::BlackQueen,
            (King, White) => Figure::WhiteKing,
            (King, Black) => Figure::BlackKing,
            (Empty, _) => Figure::Empty,
        }
    }
}
