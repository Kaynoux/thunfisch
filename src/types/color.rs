#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}
impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
