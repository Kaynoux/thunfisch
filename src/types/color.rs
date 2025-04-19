use std::ops::Not;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
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
