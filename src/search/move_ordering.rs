use crate::prelude::*;
use crate::search::mvv_lva::MVV_LVA_TABLE;
use std::cmp::Reverse;

pub fn order_moves(moves: &mut Vec<EncodedMove>, board: &Board) {
    moves.sort_unstable_by_key(|encoded_mv| {
        let mv = encoded_mv.decode();
        let mv_lva_score = if mv.is_capture {
            let attacker_idx = (board.pieces[mv.from.0] as usize) / 2;
            let victim_idx = (board.pieces[mv.to.0] as usize) / 2;
            MVV_LVA_TABLE[attacker_idx][victim_idx]
        } else {
            0i32
        };
        // sort descending by highest value first
        Reverse(mv_lva_score)
    });
}
