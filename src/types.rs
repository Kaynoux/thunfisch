#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Empty,
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
    None,
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
            Color::None => Color::None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub piece: Piece,
    pub color: Color,
    pub position: Position,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field_symbol = match (self.piece, self.color) {
            (Piece::Empty, Color::None) => " ",
            (Piece::Pawn, Color::Black) => "p",
            (Piece::Knight, Color::Black) => "n",
            (Piece::Rook, Color::Black) => "r",
            (Piece::Bishop, Color::Black) => "b",
            (Piece::Queen, Color::Black) => "q",
            (Piece::King, Color::Black) => "k",
            (Piece::Pawn, Color::White) => "P",
            (Piece::Knight, Color::White) => "N",
            (Piece::Rook, Color::White) => "R",
            (Piece::Bishop, Color::White) => "B",
            (Piece::Queen, Color::White) => "Q",
            (Piece::King, Color::White) => "K",
            (_, _) => "E",
        };
        write!(f, "{}", field_symbol)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn is_within_bounds(&self) -> bool {
        self.x >= 0 && self.x <= 7 && self.y >= 0 && self.y <= 7
    }
}

impl std::ops::Add<Move> for Position {
    type Output = Position;
    fn add(self, offset: Move) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub x: i32,
    pub y: i32,
}
