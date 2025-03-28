use crate::types::{Field, Position};

pub fn get_field(board: [[Field; 8]; 8], position: Position) -> Field {
    board[position.x as usize][position.y as usize]
}
