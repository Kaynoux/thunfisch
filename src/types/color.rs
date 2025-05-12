use std::ops::Not;
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White = 0usize,
    Black = 1usize,
}

impl Color {
    pub fn to_polyglot(self) -> usize {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl Not for Color {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
