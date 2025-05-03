#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPiece {
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

impl ColorPiece {
    pub const fn from_idx(idx: usize) -> ColorPiece {
        match idx {
            0 => ColorPiece::WhitePawn,
            1 => ColorPiece::BlackPawn,
            2 => ColorPiece::WhiteKnight,
            3 => ColorPiece::BlackKnight,
            4 => ColorPiece::WhiteBishop,
            5 => ColorPiece::BlackBishop,
            6 => ColorPiece::WhiteRook,
            7 => ColorPiece::BlackRook,
            8 => ColorPiece::WhiteQueen,
            9 => ColorPiece::BlackQueen,
            10 => ColorPiece::WhiteKing,
            11 => ColorPiece::BlackKing,
            _ => ColorPiece::Empty,
        }
    }
}
