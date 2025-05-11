use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct DecodedMove {
    pub from: Square,
    pub to: Square,
    pub mv_type: MoveType,
}

impl DecodedMove {
    pub fn to_coords(self) -> String {
        let from = self.from.to_bit().to_coords();
        let to = self.to.to_bit().to_coords();

        match self.mv_type.to_promotion_piece() {
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

        let mut mv_type = MoveType::Quiet;

        let from_str = &move_str[0..4];
        let to_str = &move_str[2..4];

        let from_pos =
            Bit::from_coords(from_str).expect(&format!("Invalid from-coords '{}'", from_str));
        let to_pos = Bit::from_coords(to_str).expect(&format!("Invalid to-coords '{}'", to_str));

        let from_idx = from_pos.to_square();
        let to_idx = to_pos.to_square();
        let from_piece = board.get_piece_at_position(from_idx);
        let to_piece = board.get_piece_at_position(to_idx);

        if to_pos.is_enemy(board, board.current_color) {
            mv_type = MoveType::Capture;
        }

        if from_piece == King && ((from_pos.to_x() as isize) - (to_pos.to_x() as isize)) == 2 {
            mv_type = MoveType::QueenCastle;
        } else if from_piece == King
            && ((from_pos.to_x() as isize) - (to_pos.to_x() as isize)) == -2
        {
            mv_type = MoveType::KingCastle;
        }

        if from_piece == Pawn && from_pos.to_y().abs_diff(to_pos.to_y()) == 1 && to_piece == Empty {
            mv_type = MoveType::EpCapture;
        }

        let promotion = if move_str.len() == 5 {
            let prom_char = move_str.chars().nth(4).unwrap();
            Some(
                Piece::from_char(prom_char)
                    .expect(&format!("Invalid promotion piece '{}'", prom_char)),
            )
        } else {
            None
        };

        if let Some(prom) = promotion {
            if mv_type == MoveType::Capture {
                match prom {
                    Queen => mv_type = MoveType::QueenPromoCapture,
                    Rook => mv_type = MoveType::RookPromoCapture,
                    Bishop => mv_type = MoveType::BishopPromoCapture,
                    Knight => mv_type = MoveType::KnightPromoCapture,
                    _ => {}
                }
            } else {
                match prom {
                    Queen => mv_type = MoveType::QueenPromo,
                    Rook => mv_type = MoveType::RookPromo,
                    Bishop => mv_type = MoveType::BishopPromo,
                    Knight => mv_type = MoveType::KnightPromo,
                    _ => {}
                }
            }
        }

        if from_piece == Pawn && from_pos.to_y().abs_diff(to_pos.to_y()) == 2 {
            mv_type = MoveType::DoubleMove
        }

        DecodedMove {
            from: from_idx,
            to: to_idx,
            mv_type,
        }
    }

    pub const fn encode(self) -> EncodedMove {
        let from_idx = self.from.0 as u16;
        let to_idx = self.to.0 as u16;
        EncodedMove(from_idx as u16 | (to_idx) << 6 | (self.mv_type as u16))
    }
}
