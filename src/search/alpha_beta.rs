use crate::prelude::*;
use crate::search::move_ordering_mvv_lva::MVV_LVA_TABLE;
use crate::search::search_info::SearchInfo;
use std::cmp::Reverse;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

const MATE_SCORE: i32 = 100000;

/// Modified Mini Max algorithm which always picks the best move for each side until a given depth is reached and the evaluates the outcomes to pick the best move at the first layer
pub fn alpha_beta(
    board: &Board,
    depth: usize,
    mut alpha: i32,
    beta: i32,
    stop: &Arc<AtomicBool>,
    search_info: &mut SearchInfo,
) -> (Option<ChessMove>, i32) {
    if depth == 0 {
        return (None, board.evaluate());
    }

    // if search time is over this statement polls the corresponding bool every 1024 nodes and cancels the node if time is over
    if search_info.total_parent_nodes % 1024 == 0 && stop.load(Ordering::Relaxed) {
        search_info.timeout_occurred = true;
        return (None, 0);
    }

    search_info.total_parent_nodes += 1;

    // set the best evaluation very low to begin with
    let mut best_eval = -MATE_SCORE * 2;
    let mut best_move: Option<ChessMove> = None;

    let mut moves = board.get_legal_moves().1;

    // returns the mate score (very low) when in check but adds the depth to give a later check a better eval
    if moves.is_empty() {
        // !!!CHECKMATE MISSING
        return (None, -MATE_SCORE + (depth as i32));
    }

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

    //println!("{}", moves.len());
    for mv in moves {
        let mut bc = board.clone();
        bc.make_move(&mv);
        let eval = alpha_beta(&bc, depth - 1, -beta, -alpha, stop, search_info).1;
        let eval = -eval;
        if eval > best_eval {
            best_eval = eval;
            best_move = Some(mv);
            if eval > alpha {
                alpha = eval;
            }
        }
        search_info.total_nodes += 1;
        if eval >= beta {
            return (best_move, best_eval);
        }
    }

    (best_move, best_eval)
}
