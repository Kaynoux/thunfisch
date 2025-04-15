use crate::types::board::Board;
use crate::types::color::Color;
#[derive(Debug, Copy, Clone)]
pub struct Position(pub u64);

impl Position {
    pub fn to_index(&self) -> usize {
        self.0.trailing_zeros() as usize
    }

    pub fn is_position_empty(self, board: &Board) -> bool {
        board.empty_pieces.is_position_set(self)
    }

    pub fn is_friendly(self, board: &Board, color: Color) -> bool {
        (color == Color::Black && board.black_pieces.is_position_set(self))
            || (color == Color::White && board.white_pieces.is_position_set(self))
    }
}
