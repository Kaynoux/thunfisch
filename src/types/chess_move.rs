use crate::prelude::{Board, Position};
use std::fmt;

use super::piece::Piece;

#[derive(Copy, Clone)]
pub struct ChessMove {
    pub from: Position,
    pub to: Position,
    pub is_capture: bool,
    pub is_double_move: bool,
    pub is_promotion: bool,
    pub is_en_passant: bool,
    pub is_castle: bool,
    pub promotion: Piece,
    pub captured: Piece,
}

impl fmt::Debug for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}->{:?}", self.from, self.to)
    }
}

impl ChessMove {
    pub fn to_coords(self) -> String {
        let from = self.from.to_coords();
        let to = self.to.to_coords();
        if self.is_promotion {
            format!("{}{}{}", from, to, self.promotion.to_lowercase_char())
        } else {
            format!("{}{}", from, to)
        }
    }

    pub fn from_coords(move_str: String, board: &Board) -> ChessMove {
        // 4 or 5 character string are valid (5 because of promotion)
        assert!(
            move_str.len() == 4 || move_str.len() == 5,
            "Invalid move string '{}', expected 4 or 5 chars",
            move_str
        );

        let from_str = &move_str[0..4];
        let to_str = &move_str[2..4];

        let from =
            Position::from_coords(from_str).expect(&format!("Invalid from-coords '{}'", from_str));
        let to = Position::from_coords(to_str).expect(&format!("Invalid to-coords '{}'", to_str));

        let from_piece = board.get_piece_and_color_at_position(from).0;
        let to_piece = board.get_piece_and_color_at_position(to).0;

        let mut capture_piece = to_piece;
        let mut is_capture = match to_piece {
            Piece::Empty => false,
            _ => true,
        };

        let is_castle = if from_piece == Piece::King && (from.to_x() - to.to_x()).abs() == 2 {
            true
        } else {
            false
        };

        let is_en_passant = if from_piece == Piece::Pawn
            && (from.to_y() - to.to_y()) == 1
            && to_piece == Piece::Empty
        {
            is_capture = true;
            capture_piece = Piece::Pawn;
            true
        } else {
            false
        };

        let (is_promotion, promotion) = if move_str.len() == 5 {
            let prom_char = move_str.chars().nth(4).unwrap();
            (
                true,
                Piece::from_char(prom_char)
                    .expect(&format!("Invalid promotion piece '{}'", prom_char)),
            )
        } else {
            (false, Piece::Empty)
        };

        let is_double_move = if from_piece == Piece::Pawn && (from.to_y() - to.to_y()).abs() == 2 {
            true
        } else {
            false
        };

        ChessMove {
            from: from,
            to: to,
            is_capture: is_capture,
            is_double_move: is_double_move,
            is_promotion: is_promotion,
            is_en_passant: is_en_passant,
            is_castle: is_castle,
            promotion: promotion,
            captured: capture_piece,
        }
    }
}
