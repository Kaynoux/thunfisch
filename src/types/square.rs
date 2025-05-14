use crate::prelude::{Bit, Bitboard};
use std::ops::Index;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Square(pub usize);

impl Square {
    #[inline(always)]
    pub const fn to_bit(self) -> Bit {
        Bit(1u64 << self.0)
    }

    #[inline(always)]
    pub const fn to_bitboard(self) -> Bitboard {
        Bitboard(1u64 << self.0)
    }

    #[inline(always)]
    pub const fn next(&mut self) {
        self.0 += 1;
    }

    /// Quick way to access array index by Square
    #[inline(always)]
    pub const fn i(self) -> usize {
        self.0
    }

    pub const fn from_xy(x: usize, y: usize) -> Square {
        Square(y * 8 + x)
    }

    pub const fn y(self) -> usize {
        self.0 / 8
    }

    pub const fn x(self) -> usize {
        self.0 & 7 // equivalent to % 8 for positiv numbers
    }

    /// Converts UCI notation to square
    #[inline(always)]
    pub fn from_coords(coords: &str) -> Option<Square> {
        let (c1, c2) = match Bit::get_first_two_string_chars(coords) {
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

        Some(Square((y * 8 + x) as usize))
    }
}

impl<T, const N: usize> Index<Square> for [T; N] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        &self[index.i()]
    }
}
