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
}
