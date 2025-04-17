use crate::prelude::*;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

/// the bits the are set represent a position on the board with the bit being the index of the chess position
/// Counting begins bottom left
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    #[inline(always)]
    pub fn is_position_set(self, position: Position) -> bool {
        (self & position) != Bitboard(0)
    }

    pub fn pop_lsb_position(&mut self) -> Position {
        let pos = Position(self.0 & self.0.wrapping_neg());
        self.0 &= self.0 - 1;
        pos
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
        self.0 & rhs.0;
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

impl BitAnd<Bitboard> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign<Bitboard> for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 & rhs.0;
    }
}
