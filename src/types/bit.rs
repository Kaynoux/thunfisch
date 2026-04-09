use crate::prelude::*;
use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr},
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Bit(pub u64);

impl fmt::Debug for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = self.to_xy();
        write!(f, "[{x},{y}]")
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

impl Bit {
    #[inline]
    pub const fn to_square(self) -> Square {
        Square(self.0.trailing_zeros() as usize)
    }

    #[inline]
    pub const fn to_xy(self) -> (usize, usize) {
        POSITION_XY[self.to_square().0]
    }

    #[inline]
    pub const fn to_x(self) -> usize {
        POSITION_X[self.to_square().0]
    }

    #[inline]
    pub const fn to_y(self) -> usize {
        POSITION_Y[self.to_square().0]
    }

    #[inline]
    pub fn is_position_empty(self, board: &Board) -> bool {
        board.empty().is_position_set(self)
    }

    #[inline]
    pub fn is_friendly(self, board: &Board, color: Color) -> bool {
        (color == Black && board.color_bbs(Black).is_position_set(self))
            || (color == White && board.color_bbs(White).is_position_set(self))
    }

    #[inline]
    pub fn is_enemy(self, board: &Board, color: Color) -> bool {
        (color == White && board.color_bbs(Black).is_position_set(self))
            || (color == Black && board.color_bbs(White).is_position_set(self))
    }

    #[inline]
    pub const fn get_offset_pos(self, dx: isize, dy: isize) -> Self {
        let pos_idx = self.to_square().0.cast_signed();
        let new_x: isize = pos_idx % 8 + dx;
        let new_y: isize = pos_idx / 8 + dy;
        if new_x >= 0 && new_x <= 7 && new_y >= 0 && new_y <= 7 {
            let new_idx = new_y * 8 + new_x;
            return Self(1u64 << new_idx);
        }
        Self(0)
    }

    #[inline]
    pub fn get_first_two_string_chars(s: &str) -> Option<(char, char)> {
        let mut iter = s.chars();
        match (iter.next(), iter.next()) {
            (Some(c1), Some(c2)) => Some((c1, c2)),
            _ => None,
        }
    }

    #[inline]
    pub fn from_coords(coords: &str) -> Option<Self> {
        Square::from_coords(coords).map(Square::to_bit)
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub fn to_coords(self) -> String {
        let (x, y) = self.to_xy();
        let file = (b'a' + x as u8) as char;
        let rank = (b'1' + y as u8) as char;
        format!("{file}{rank}")
    }

    #[inline]
    pub const fn from_xy(x: isize, y: isize) -> Self {
        Square((x + (y * 8)).cast_unsigned()).to_bit()
    }

    #[inline]
    pub const fn to_bb(self) -> Bitboard {
        Bitboard(self.0)
    }
}

impl Shl<isize> for Bit {
    type Output = Self;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[inline]
    fn shl(self, shift: isize) -> Self::Output {
        if shift < 0 {
            // Interpret negative shift as right shift
            return Self(self.0 >> ((-shift) as u32));
        }
        Self(self.0 << (shift as u32))
    }
}

impl Shr<isize> for Bit {
    type Output = Self;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[inline]
    fn shr(self, shift: isize) -> Self::Output {
        if shift < 0 {
            // Interpret negative shift as left shift
            return Self(self.0 << ((-shift) as u32));
        }
        Self(self.0 >> (shift as u32))
    }
}

impl BitAnd<Bitboard> for Bit {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr<Self> for Bit {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Not for Bit {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAndAssign<Self> for Bit {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign<Self> for Bit {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl From<Bitboard> for Bit {
    fn from(bitboard: Bitboard) -> Self {
        Self(bitboard.0)
    }
}
