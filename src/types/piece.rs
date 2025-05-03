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
            (Piece::Empty, _) => ' ',
            (Piece::Pawn, Color::White) => 'P',
            (Piece::Pawn, Color::Black) => 'p',
            (Piece::Knight, Color::White) => 'N',
            (Piece::Knight, Color::Black) => 'n',
            (Piece::Bishop, Color::White) => 'B',
            (Piece::Bishop, Color::Black) => 'b',
            (Piece::Rook, Color::White) => 'R',
            (Piece::Rook, Color::Black) => 'r',
            (Piece::Queen, Color::White) => 'Q',
            (Piece::Queen, Color::Black) => 'q',
            (Piece::King, Color::White) => 'K',
            (Piece::King, Color::Black) => 'k',
        }
    }

    pub fn to_unicode_char(self, color: Color) -> char {
        match (self, color) {
            (Piece::Empty, _) => '.',
            (Piece::Pawn, Color::Black) => '♙',
            (Piece::Pawn, Color::White) => '♟',
            (Piece::Knight, Color::Black) => '♘',
            (Piece::Knight, Color::White) => '♞',
            (Piece::Bishop, Color::Black) => '♗',
            (Piece::Bishop, Color::White) => '♝',
            (Piece::Rook, Color::Black) => '♖',
            (Piece::Rook, Color::White) => '♜',
            (Piece::Queen, Color::Black) => '♕',
            (Piece::Queen, Color::White) => '♛',
            (Piece::King, Color::Black) => '♔',
            (Piece::King, Color::White) => '♚',
        }
    }

    pub fn from_char(piece_char: char) -> Option<Piece> {
        match piece_char {
            'p' => Some(Piece::Pawn),
            'n' => Some(Piece::Knight),
            'b' => Some(Piece::Bishop),
            'r' => Some(Piece::Rook),
            'q' => Some(Piece::Queen),
            'k' => Some(Piece::King),
            _ => None,
        }
    }

    pub fn to_lowercase_char(self) -> char {
        match self {
            Piece::Empty => ' ',
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        }
    }

    pub fn to_color_piece(self, color: Color) -> ColorPiece {
        ColorPiece::from_idx((self as usize) * 2 + (color as usize))
    }
}
