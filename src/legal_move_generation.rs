use crate::prelude::*;
use crate::pseudo_legal_move_generation;

pub fn generate_legal_moves(board: &Board, color: Color, moves: &mut Vec<ChessMove>) {
    let moves_bitboard = match color {
        Color::Black => pseudo_legal_move_generation::get_all_black_moves(board, moves),
        Color::White => pseudo_legal_move_generation::get_all_white_moves(board, moves),
    };

    // only retain moves where king is not in check
    moves.retain(|mv| {
        let mut bc = board.clone();
        bc.make_move(mv);

        // generate counter moves
        let mut counter_moves: Vec<ChessMove> = Vec::new();
        let counter_positions = match color {
            Color::Black => {
                pseudo_legal_move_generation::get_all_white_moves(&bc, &mut counter_moves)
            }
            Color::White => {
                pseudo_legal_move_generation::get_all_black_moves(&bc, &mut counter_moves)
            }
        };

        // where is king?
        let king_pos = match color {
            Color::Black => bc.black_king,
            Color::White => bc.white_king,
        };

        // only keep position if king is not in counter attack positions
        counter_positions & king_pos == Bitboard(0)
    });
}
