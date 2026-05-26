use crate::prelude::*;
use std::ops::Not;
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White = 0usize,
    Black = 1usize,
}

impl Color {
    /// My color number representation uses opposite values as in the polyglot values so I need to this method
    /// to lazy to change this now
    pub(crate) const fn to_polyglot(self) -> usize {
        match self {
            Black => 0,
            White => 1,
        }
    }


    #[inline]
    pub(crate) fn from_usize(val: usize) -> Self {
        match val {
            0 => Self::White,
            1 => Self::Black,
            _ => unreachable!(),
        }
    }
}

impl Not for Color {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Black => White,
            White => Black,
        }
    }
}
