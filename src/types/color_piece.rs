#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPiece {
    Empty = 0,
    WhitePawn = 1,
    BlackPawn = 2,
    WhiteKnight = 3,
    BlackKnight = 4,
    WhiteBishop = 5,
    BlackBishop = 6,
    WhiteRook = 7,
    BlackRook = 8,
    WhiteQueen = 9,
    BlackQueen = 10,
    WhiteKing = 11,
    BlackKing = 12,
}
