// Enums get compiled to Integers so they have no performance overhead
#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Empty,
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}
