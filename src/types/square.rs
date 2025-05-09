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

    /// Uses as index as usize
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
}

impl<T, const N: usize> Index<Square> for [T; N] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        &self[index.i()]
    }
}
