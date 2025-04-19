use crate::prelude::Position;
use std::fmt;

use super::piece::Piece;

#[derive(Copy, Clone)]
pub struct ChessMove {
    pub from: Position,
    pub to: Position,
    pub is_capture: bool,
    pub is_double_move: bool,
    pub is_promotion: bool,
    pub is_en_passant: bool,
    pub is_castle: bool,
    pub promotion: Piece,
    pub captured: Piece,
}

impl fmt::Debug for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}->{:?}", self.from, self.to)
    }
}
