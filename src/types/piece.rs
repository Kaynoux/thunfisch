use crate::{move_generator::magics::ROOK_MAGICS, prelude::*};

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
    pub fn to_polyglot(self) -> usize {
        match self {
            Pawn => 0,
            Knight => 1,
            Bishop => 2,
            Rook => 3,
            Queen => 4,
            King => 5,
            Empty => usize::MAX,
        }
    }
    /// Returns the correct FIN symbol by matching the piece together with the provided color.
    pub fn to_fin_char(self, color: Color) -> char {
        match (self, color) {
            (Empty, _) => ' ',
            (Pawn, White) => 'P',
            (Pawn, Black) => 'p',
            (Knight, White) => 'N',
            (Knight, Black) => 'n',
            (Bishop, White) => 'B',
            (Bishop, Black) => 'b',
            (Rook, White) => 'R',
            (Rook, Black) => 'r',
            (Queen, White) => 'Q',
            (Queen, Black) => 'q',
            (King, White) => 'K',
            (King, Black) => 'k',
        }
    }

    pub fn to_unicode_char(self, color: Color) -> char {
        match (self, color) {
            (Empty, _) => '.',
            (Pawn, Black) => '♙',
            (Pawn, White) => '♟',
            (Knight, Black) => '♘',
            (Knight, White) => '♞',
            (Bishop, Black) => '♗',
            (Bishop, White) => '♝',
            (Rook, Black) => '♖',
            (Rook, White) => '♜',
            (Queen, Black) => '♕',
            (Queen, White) => '♛',
            (King, Black) => '♔',
            (King, White) => '♚',
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

    pub const fn to_color_piece(self, color: Color) -> Figure {
        Figure::from_idx((self as usize) * 2 + (color as usize))
    }
}
