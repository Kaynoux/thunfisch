use crate::prelude::*;
use crate::search::mvv_lva::MVV_LVA_TABLE;
use std::cmp::Reverse;

pub fn order_moves(moves: &mut Vec<OldChessMove>, board: &Board) {
    moves.sort_unstable_by_key(|mv| {
        let mv_lva_score = if mv.is_capture {
            let attacker_idx = board.get_piece_idx_at_position(mv.from);
            let victim_idx = board.get_piece_idx_at_position(mv.to);
            MVV_LVA_TABLE[attacker_idx][victim_idx]
        } else {
            0i32
        };
        // sort descending by highest value first
        Reverse(mv_lva_score)
    });
}
