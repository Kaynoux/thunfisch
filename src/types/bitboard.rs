use super::position::Position;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

/// the bits the are set represent a position on the board with the bit being the index of the chess position
/// Counting begins bottom left
#[derive(Debug, Copy, Clone)]
pub struct Bitboard(pub u64);

impl Bitboard {
    #[inline(always)]
    pub fn is_position_set(self, position: Position) -> bool {
        (self & position) != 0
    }
}

impl PartialEq<u64> for Bitboard {
    #[inline(always)]
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
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
