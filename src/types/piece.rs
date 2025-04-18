use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    /// Returns the correct FIN symbol by matching the piece together with the provided color.
    pub fn get_fin_symbol(self, color: Color) -> char {
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

    pub fn get_unicode_symbol(self, color: Color) -> char {
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
}
