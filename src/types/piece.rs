use crate::prelude::*;

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
    Empty = 6,
}

impl Piece {
    /// Returns the correct FIN symbol by matching the piece together with the provided color.
    pub fn to_fin_char(self, color: Color) -> char {
        match (self, color) {
            (Empty, _) => ' ',
            (Pawn, Color::White) => 'P',
            (Pawn, Color::Black) => 'p',
            (Knight, Color::White) => 'N',
            (Knight, Color::Black) => 'n',
            (Bishop, Color::White) => 'B',
            (Bishop, Color::Black) => 'b',
            (Rook, Color::White) => 'R',
            (Rook, Color::Black) => 'r',
            (Queen, Color::White) => 'Q',
            (Queen, Color::Black) => 'q',
            (King, Color::White) => 'K',
            (King, Color::Black) => 'k',
        }
    }

    pub fn to_unicode_char(self, color: Color) -> char {
        match (self, color) {
            (Empty, _) => '.',
            (Pawn, Color::Black) => '♙',
            (Pawn, Color::White) => '♟',
            (Knight, Color::Black) => '♘',
            (Knight, Color::White) => '♞',
            (Bishop, Color::Black) => '♗',
            (Bishop, Color::White) => '♝',
            (Rook, Color::Black) => '♖',
            (Rook, Color::White) => '♜',
            (Queen, Color::Black) => '♕',
            (Queen, Color::White) => '♛',
            (King, Color::Black) => '♔',
            (King, Color::White) => '♚',
        }
    }

    pub fn from_char(piece_char: char) -> Option<Piece> {
        match piece_char {
            'p' => Some(Pawn),
            'n' => Some(Knight),
            'b' => Some(Bishop),
            'r' => Some(Rook),
            'q' => Some(Queen),
            'k' => Some(King),
            _ => None,
        }
    }

    pub fn to_lowercase_char(self) -> char {
        match self {
            Empty => ' ',
            Pawn => 'p',
            Knight => 'n',
            Bishop => 'b',
            Rook => 'r',
            Queen => 'q',
            King => 'k',
        }
    }

    pub fn to_color_piece(self, color: Color) -> Figure {
        Figure::from_idx((self as usize) * 2 + (color as usize))
    }
}
