use crate::prelude::*;
use crate::pseudo_legal_move_generation;

impl Board {
    pub fn generate_legal_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let color = self.current_color;
        pseudo_legal_move_generation::get_all_moves(self, color, &mut moves);

        // only retain moves where king is not in check after being in check and follows all rules when castling
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

            // This part gets run if the king is currently in check so it needs to be resolved
            bc.make_move(mv);

            // generate counter moves for this move
            let mut counter_moves_after_move: Vec<ChessMove> = Vec::new();
            let counter_positions_after_move = pseudo_legal_move_generation::get_all_moves(
                &bc,
                !color,
                &mut counter_moves_after_move,
            );

            let king_pos_after_move = match color {
                Color::Black => bc.black_king,
                Color::White => bc.white_king,
            };

            // Keep move if not in check and throw away if it is
            counter_positions_after_move & king_pos_after_move == Bitboard(0)
        });

        moves
    }
}
