use crate::prelude::*;
use crate::pseudo_legal_move_generation;

impl Board {
    pub fn generate_legal_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let color = self.current_color;
        pseudo_legal_move_generation::get_all_moves(self, color, &mut moves);

        // only retain moves where king is not in check
        moves.retain(|mv| {
            let mut bc = self.clone();

            if mv.is_castle {
                let mut counter_moves: Vec<ChessMove> = Vec::new();
                let counter_positions =
                    pseudo_legal_move_generation::get_all_moves(&bc, !color, &mut counter_moves);

                const WHITE_CASTLE_LEFT_MOVE: (Position, Position) =
                    (Position::from_idx(4), Position::from_idx(2));
                const WHITE_CASTLE_RIGHT_MOVE: (Position, Position) =
                    (Position::from_idx(4), Position::from_idx(6));
                const BLACK_CASTLE_LEFT_MOVE: (Position, Position) =
                    (Position::from_idx(60), Position::from_idx(58));
                const BLACK_CASTLE_RIGHT_MOVE: (Position, Position) =
                    (Position::from_idx(60), Position::from_idx(62));
                match (mv.from, mv.to) {
                    WHITE_CASTLE_LEFT_MOVE => {
                        const MASK_WHITE_LEFT: Bitboard = Bitboard::from_idx([2usize, 3usize]);
                        if counter_positions & MASK_WHITE_LEFT != Bitboard(0) {
                            return false;
                        }
                    }
                    WHITE_CASTLE_RIGHT_MOVE => {
                        const MASK_WHITE_RIGHT: Bitboard = Bitboard::from_idx([5usize, 6usize]);
                        if counter_positions & MASK_WHITE_RIGHT != Bitboard(0) {
                            return false;
                        }
                    }
                    BLACK_CASTLE_LEFT_MOVE => {
                        const MASK_BLACK_LEFT: Bitboard = Bitboard::from_idx([58usize, 59usize]);
                        if counter_positions & MASK_BLACK_LEFT != Bitboard(0) {
                            return false;
                        }
                    }
                    BLACK_CASTLE_RIGHT_MOVE => {
                        const MASK_BLACK_RIGHT: Bitboard = Bitboard::from_idx([61usize, 62usize]);
                        if counter_positions & MASK_BLACK_RIGHT != Bitboard(0) {
                            return false;
                        }
                    }
                    _ => {}
                }
            }

            bc.make_move(mv);

            // generate counter moves
            let mut counter_moves: Vec<ChessMove> = Vec::new();
            let counter_positions =
                pseudo_legal_move_generation::get_all_moves(&bc, !color, &mut counter_moves);

            // where is king?
            let king_pos = match color {
                Color::Black => bc.black_king,
                Color::White => bc.white_king,
            };

            // only keep position if king is not in counter attack positions
            counter_positions & king_pos == Bitboard(0)
        });

        moves
    }
}
