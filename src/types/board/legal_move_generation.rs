use crate::prelude::*;
use crate::pseudo_legal_move_generation;

impl Board {
    pub fn get_legal_moves(&self, only_captures: bool) -> Vec<OldChessMove> {
        let color = self.current_color;
        let mut moves = Vec::with_capacity(128);
        let _ = pseudo_legal_move_generation::get_all_moves(&mut moves, self, color);

        // only retain moves where king is not in check after being in check and follows all rules when castling
        moves.retain(|mv| {
            if only_captures && !mv.is_capture {
                return false;
            }

            let mut bc = self.clone();

            let enemy_king_pos = self.get_king_pos(!color);
            if mv.to == enemy_king_pos {
                // winning move is invalid
                return false;
            }

            if mv.is_castle {
                let counter_positions = pseudo_legal_move_generation::get_all_attacks(&bc, !color);

                let king_pos = match color {
                    Color::Black => bc.black_king,
                    Color::White => bc.white_king,
                };

                // castling not allowed if king is in check before castling
                if counter_positions & king_pos != Bitboard(0) {
                    return false;
                };

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
            let counter_positions_after_move =
                pseudo_legal_move_generation::get_all_attacks(&bc, !color);

            let king_pos_after_move = match color {
                Color::Black => bc.black_king,
                Color::White => bc.white_king,
            };

            // Keep move if not in check and throw away if it is
            if counter_positions_after_move & king_pos_after_move != Bitboard(0) {
                return false;
            }

            true
        });

        moves
    }
}
