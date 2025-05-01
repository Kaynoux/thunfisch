use crate::prelude::*;
use std::{
    arch::asm,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

/// the bits the are set represent a position on the board with the bit being the index of the chess position
/// Counting begins bottom left
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    #[inline(always)]
    pub fn is_position_set(self, position: Position) -> bool {
        (self & position) != Bitboard(0)
    }

    #[inline(always)]
    pub fn pop_lsb_position(&mut self) -> Option<Position> {
        if self.0 == 0 {
            None
        } else {
            let pos = Position(self.0 & self.0.wrapping_neg());
            self.0 &= self.0 - 1;
            Some(pos)
        }
    }

    #[inline(always)]
    pub fn pop_lsb_asm(&mut self) -> Option<Position> {
        let mut lsb_index: u64;
        unsafe {
            asm!(
                "bsf {0}, {1}",
                out(reg) lsb_index,
                in(reg) self.0,
            );
        }
        if lsb_index != 64 {
            self.0 &= self.0 - 1;
            Some(Position(lsb_index))
        } else {
            None
        }
    }

    pub fn iter(&mut self) -> impl Iterator<Item = Position> + '_ {
        std::iter::from_fn(move || self.pop_lsb_position())
    }

    #[inline(always)]
    pub const fn from_idx<const N: usize>(indexes: [usize; N]) -> Self {
        let mut bitboard = Bitboard(0);
        let mut i = 0;
        while i < N {
            bitboard = Bitboard(bitboard.0 | (1u64 << indexes[i]));
            i += 1;
        }
        bitboard
    }
}

impl Not for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl BitOr<Position> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Position) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign<Position> for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Position) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<u64> for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl BitAnd<Position> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Position) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign<Position> for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Position) {
        self.0 &= rhs.0;
    }
}

impl BitOr<Bitboard> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign<Bitboard> for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign<Bitboard> for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}
