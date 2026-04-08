use crate::prelude::*;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not, Sub};

/// the bits the are set represent a position on the board with the bit being the index of the chess position
/// Counting begins bottom left
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bitboard(pub u64);

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?; // Start with a newline for better formatting in some contexts
        let mut y = 7;
        while y >= 0 {
            write!(f, "{y} | ")?;
            let mut x = 0;
            while x <= 7 {
                #[allow(clippy::cast_sign_loss)]
                let idx = (y * 8 + x) as usize;
                let bit_is_set = (self.0 >> idx) & 1;
                if bit_is_set == 1 {
                    write!(f, "X ")?; // Or "1 "
                } else {
                    write!(f, ". ")?; // Or "0 "
                }
                x += 1;
            }
            y -= 1;
            writeln!(f)?;
        }
        writeln!(f, "    ---------------")?; // Adjusted separator
        writeln!(f, "    0 1 2 3 4 5 6 7")?;
        writeln!(f, "-------------------")?;
        Ok(())
    }
}

impl Bitboard {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(u64::MAX);
    pub const UNSET_CHECK_MASK: Self = Self(0);
    pub const UNSET_ATTACK_MASK: Self = Self(u64::MAX);
    pub const UNSET_PINMASK: Self = Self(u64::MAX);

    #[inline]
    pub fn is_position_set(self, position: Bit) -> bool {
        (self & position) != Self(0)
    }

    pub const fn toggle(&mut self, square: Square) {
        self.0 ^= 1 << square.i();
    }

    #[inline]
    pub fn pop_lsb_position(&mut self) -> Option<Bit> {
        if *self == Self(0) {
            None
        } else {
            let pos = Bit::from(*self & self.0.wrapping_neg());
            *self &= *self - Self(1);
            Some(pos)
        }
    }

    #[inline]
    pub const fn get_count(self) -> u32 {
        self.0.count_ones()
    }

    /// Iterator for Bitboard, uses pop lsb to always return
    pub fn iter_mut(&mut self) -> impl Iterator<Item = Bit> + '_ {
        std::iter::from_fn(move || self.pop_lsb_position())
    }

    #[inline]
    pub const fn from_idx<const N: usize>(indexes: [usize; N]) -> Self {
        let mut bitboard = Self(0);
        let mut i = 0;
        while i < N {
            bitboard = Self(bitboard.0 | (1u64 << indexes[i]));
            i += 1;
        }
        bitboard
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn count(self) -> usize {
        self.0.count_ones() as usize
    }
}

impl Not for Bitboard {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitOr<Bit> for Bitboard {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Bit) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign<Bit> for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Bit) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<u64> for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl BitAnd<Bit> for Bitboard {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Bit) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitXor<Self> for Bitboard {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitAndAssign<Bit> for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Bit) {
        self.0 &= rhs.0;
    }
}

impl BitOr<Self> for Bitboard {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign<Self> for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign<Self> for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Sub for Bitboard {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: u64) -> Self::Output {
        Self(self.0 & rhs)
    }
}
