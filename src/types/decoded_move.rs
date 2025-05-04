use crate::prelude::*;

use super::encoded_move::move_flags;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DecodedMove {
    pub from: IndexPosition,
    pub to: IndexPosition,
    pub is_capture: bool,
    pub is_double_move: bool,
    pub is_ep_capture: bool,
    pub is_king_castle: bool,
    pub is_queen_castle: bool,
    pub promotion: Option<Piece>,
}

impl DecodedMove {
    pub fn to_coords(self) -> String {
        let from = self.from.to_position().to_coords();
        let to = self.to.to_position().to_coords();

        match self.promotion {
            Some(prom) => format!("{}{}{}", from, to, prom.to_lowercase_char()),
            None => format!("{}{}", from, to),
        }
    }

    pub fn from_coords(move_str: String, board: &Board) -> DecodedMove {
        // 4 or 5 character string are valid (5 because of promotion)
        assert!(
            move_str.len() == 4 || move_str.len() == 5,
            "Invalid move string '{}', expected 4 or 5 chars",
            move_str
        );

        let from_str = &move_str[0..4];
        let to_str = &move_str[2..4];

        let from_pos =
            Position::from_coords(from_str).expect(&format!("Invalid from-coords '{}'", from_str));
        let to_pos =
            Position::from_coords(to_str).expect(&format!("Invalid to-coords '{}'", to_str));

        let from_idx = from_pos.to_index();
        let to_idx = to_pos.to_index();
        let from_piece = board.get_piece_at_position(from_idx);
        let to_piece = board.get_piece_at_position(to_idx);

        let mut is_capture = to_pos.is_enemy(board, board.current_color);

        let (is_queen_castle, is_king_castle) = if from_piece == Piece::King
            && ((from_pos.to_x() as isize) - (to_pos.to_x() as isize)) == 2
        {
            (true, false)
        } else if from_piece == Piece::King
            && ((from_pos.to_x() as isize) - (to_pos.to_x() as isize)) == -2
        {
            (false, true)
        } else {
            (false, false)
        };

        let is_ep_capture = if from_piece == Piece::Pawn
            && from_pos.to_y().abs_diff(to_pos.to_y()) == 1
            && to_piece == Piece::Empty
        {
            is_capture = true;
            true
        } else {
            false
        };

        let promotion = if move_str.len() == 5 {
            let prom_char = move_str.chars().nth(4).unwrap();
            Some(
                Piece::from_char(prom_char)
                    .expect(&format!("Invalid promotion piece '{}'", prom_char)),
            )
        } else {
            None
        };

        let is_double_move =
            if from_piece == Piece::Pawn && from_pos.to_y().abs_diff(to_pos.to_y()) == 2 {
                true
            } else {
                false
            };

        DecodedMove {
            from: from_idx,
            to: to_idx,
            is_capture: is_capture,
            is_double_move: is_double_move,
            is_ep_capture,
            is_king_castle,
            is_queen_castle,
            promotion: promotion,
        }
    }

    pub fn encode(&self) -> EncodedMove {
        let flag = if let Some(piece) = self.promotion {
            match self.is_capture {
                true => match piece {
                    Piece::Queen => move_flags::QUEEN_PROMO_CAPTURE,
                    Piece::Rook => move_flags::ROOK_PROMO_CAPTURE,
                    Piece::Bishop => move_flags::BISHOP_PROMO_CAPTURE,
                    Piece::Knight => move_flags::KNIGHT_PROMO_CAPTURE,
                    _ => move_flags::QUIET, // should never occur
                },
                false => match piece {
                    Piece::Queen => move_flags::QUEEN_PROMO,
                    Piece::Rook => move_flags::ROOK_PROMO,
                    Piece::Bishop => move_flags::BISHOP_PROMO,
                    Piece::Knight => move_flags::KNIGHT_PROMO,
                    _ => move_flags::QUIET, // should never occur
                },
            }
        } else if self.is_ep_capture {
            move_flags::EP_CAPTURE
        } else if self.is_capture {
            move_flags::CAPTURE
        } else if self.is_double_move {
            move_flags::DOUBLE_MOVE
        } else if self.is_queen_castle {
            move_flags::QUEEN_CASTLE
        } else if self.is_king_castle {
            move_flags::KING_CASTLE
        } else {
            move_flags::QUIET
        };

        EncodedMove::encode(self.from.to_position(), self.to.to_position(), flag)
    }
}
