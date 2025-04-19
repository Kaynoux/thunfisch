use crate::prelude::*;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl};

#[derive(Copy, Clone, PartialEq)]
pub struct Position(pub u64);

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = self.to_xy();
        write!(f, "[{},{}]", x, y)
    }
}

impl Position {
    pub fn to_index(self) -> isize {
        self.0.trailing_zeros() as isize
    }

    pub fn to_xy(self) -> (isize, isize) {
        let idx = self.to_index();
        let x = idx % 8;
        let y = idx / 8;
        (x, y)
    }

    pub fn is_position_empty(self, board: &Board) -> bool {
        board.empty_pieces.is_position_set(self)
    }

    pub fn is_friendly(self, board: &Board, color: Color) -> bool {
        (color == Color::Black && board.black_pieces.is_position_set(self))
            || (color == Color::White && board.white_pieces.is_position_set(self))
    }

    pub fn is_enemy(self, board: &Board, color: Color) -> bool {
        (color == Color::White && board.black_pieces.is_position_set(self))
            || (color == Color::Black && board.white_pieces.is_position_set(self))
    }

    pub fn get_offset_pos(self, dx: isize, dy: isize) -> Position {
        let pos_idx = self.to_index() as isize;
        let new_x: isize = pos_idx % 8 + dx;
        let new_y: isize = pos_idx / 8 + dy;
        if new_x >= 0 && new_x <= 7 && new_y >= 0 && new_y <= 7 {
            let new_idx = new_y * 8 + new_x;
            return Position(1u64 << new_idx);
        }
        Position(0)
    }
}

impl Shl<isize> for Position {
    type Output = Self;
    #[inline(always)]
    fn shl(self, shift: isize) -> Self::Output {
        if shift < 0 {
            // Interpret negative shift as right shift
            return Position(self.0 >> ((-shift) as u32));
        }
        Position(self.0 << (shift as u32))
    }
}

impl BitAnd<Bitboard> for Position {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Position(self.0 | rhs.0)
    }
}

impl BitOr<Position> for Position {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Position) -> Self::Output {
        Position(self.0 | rhs.0)
    }
}

impl Not for Position {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self::Output {
        Position(!self.0)
    }
}

impl BitAndAssign<Position> for Position {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Position) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign<Position> for Position {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Position) {
        self.0 |= rhs.0;
    }
}
