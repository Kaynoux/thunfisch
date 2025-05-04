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

pub const POSITION_XY: [(usize, usize); 64] = {
    let mut lookup_table = [(0, 0); 64];
    let mut i = 0;
    while i < 64 {
        lookup_table[i] = ((i % 8), (i / 8));
        i += 1;
    }
    lookup_table
};

pub const POSITION_X: [usize; 64] = {
    let mut lookup_table = [0; 64];
    let mut i = 0;
    while i < 64 {
        lookup_table[i] = i % 8;
        i += 1;
    }
    lookup_table
};

pub const POSITION_Y: [usize; 64] = {
    let mut lookup_table = [0; 64];
    let mut i = 0;
    while i < 64 {
        lookup_table[i] = i / 8;
        i += 1;
    }
    lookup_table
};

impl Position {
    #[inline(always)]
    pub const fn to_index(self) -> IndexPosition {
        IndexPosition(self.0.trailing_zeros() as usize)
    }

    #[inline(always)]
    pub const fn to_xy(self) -> (usize, usize) {
        POSITION_XY[self.to_index().0]
    }

    #[inline(always)]
    pub const fn to_x(self) -> usize {
        POSITION_X[self.to_index().0]
    }

    #[inline(always)]
    pub const fn to_y(self) -> usize {
        POSITION_Y[self.to_index().0]
    }

    #[inline(always)]
    pub fn is_position_empty(self, board: &Board) -> bool {
        board.get_empty_pieces().is_position_set(self)
    }

    #[inline(always)]
    pub fn is_friendly(self, board: &Board, color: Color) -> bool {
        (color == Color::Black && board.black_pieces.is_position_set(self))
            || (color == Color::White && board.white_pieces.is_position_set(self))
    }

    #[inline(always)]
    pub fn is_enemy(self, board: &Board, color: Color) -> bool {
        (color == Color::White && board.black_pieces.is_position_set(self))
            || (color == Color::Black && board.white_pieces.is_position_set(self))
    }

    #[inline(always)]
    pub const fn get_offset_pos(self, dx: isize, dy: isize) -> Position {
        let pos_idx = self.to_index().0 as isize;
        let new_x: isize = pos_idx % 8 + dx;
        let new_y: isize = pos_idx / 8 + dy;
        if new_x >= 0 && new_x <= 7 && new_y >= 0 && new_y <= 7 {
            let new_idx = new_y * 8 + new_x;
            return Position(1u64 << new_idx);
        }
        Position(0)
    }

    #[inline(always)]
    pub fn get_first_two_string_chars(s: &str) -> Option<(char, char)> {
        let mut iter = s.chars();
        match (iter.next(), iter.next()) {
            (Some(c1), Some(c2)) => Some((c1, c2)),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn from_coords(coords: &str) -> Option<Position> {
        let (c1, c2) = match Position::get_first_two_string_chars(coords) {
            Some(c1c2) => c1c2,
            None => return None,
        };

        let x: isize = match c1 {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return None,
        };

        let y: isize = match c2 {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return None,
        };

        Some(IndexPosition((y * 8 + x) as usize).to_position())
    }

    #[inline(always)]
    pub fn to_coords(self) -> String {
        let (x, y) = self.to_xy();
        let file = (b'a' + x as u8) as char;
        let rank = (b'1' + y as u8) as char;
        format!("{}{}", file, rank)
    }

    #[inline(always)]
    pub const fn from_xy(x: isize, y: isize) -> Position {
        IndexPosition((x + (y * 8)) as usize).to_position()
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
