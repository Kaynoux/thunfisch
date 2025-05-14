use crate::prelude::*;
use std::fmt;
use std::ops::BitXor;
use std::{
    arch::asm,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

/// the bits the are set represent a position on the board with the bit being the index of the chess position
/// Counting begins bottom left
#[derive(Copy, Clone, PartialEq)]
pub struct Bitboard(pub u64);

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?; // Start with a newline for better formatting in some contexts
        let mut y = 7;
        while y >= 0 {
            write!(f, "{} | ", y)?;
            let mut x = 0;
            while x <= 7 {
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
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);
    #[inline(always)]
    pub fn is_position_set(self, position: Bit) -> bool {
        (self & position) != Bitboard(0)
    }

    pub fn toggle(&mut self, square: Square) {
        self.0 ^= 1 << square.i();
    }

    #[inline(always)]
    pub fn pop_lsb_position(&mut self) -> Option<Bit> {
        if self.0 == 0 {
            None
        } else {
            let pos = Bit(self.0 & self.0.wrapping_neg());
            self.0 &= self.0 - 1;
            Some(pos)
        }
    }

    #[inline(always)]
    pub fn pop_lsb_asm(&mut self) -> Option<Bit> {
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
            Some(Bit(lsb_index))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_count(&self) -> u32 {
        self.0.count_ones()
    }

    /// Iterator for Bitboard, uses pop lsb to always return
    pub fn iter_mut(&mut self) -> impl Iterator<Item = Bit> + '_ {
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

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub const fn count(self) -> usize {
        self.0.count_ones() as usize
    }
}

impl Not for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl BitOr<Bit> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Bit) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign<Bit> for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Bit) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<u64> for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl BitAnd<Bit> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Bit) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitXor<Bitboard> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitAndAssign<Bit> for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Bit) {
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
