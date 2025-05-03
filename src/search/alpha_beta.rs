use crate::prelude::*;
use crate::search::move_ordering;
use crate::search::search_info::SearchInfo;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

const MATE_SCORE: i32 = 100000;
const MAX_QUIESCENCE_SEARCH_DEPTH: usize = 8;

/// Modified Mini Max algorithm which always picks the best move for each side until a given depth is reached and the evaluates the outcomes to pick the best move at the first layer
pub fn alpha_beta(
    board: &Board,
    depth: usize,
    mut alpha: i32,
    beta: i32,
    stop: &Arc<AtomicBool>,
    search_info: &mut SearchInfo,
) -> (Option<EncodedMove>, i32) {
    search_info.total_nodes += 1;
    if depth == 0 {
        return (
            None,
            quiescence_search(
                board,
                alpha,
                beta,
                MAX_QUIESCENCE_SEARCH_DEPTH,
                stop,
                search_info,
            ),
        );
    }

    // if search time is over this statement polls the corresponding bool every 1024 nodes and cancels the node if time is over
    if search_info.total_nodes % 1024 == 0 && stop.load(Ordering::Relaxed) {
        search_info.timeout_occurred = true;
        return (None, 0);
    }

    // set the best evaluation very low to begin with
    let mut best_eval = i32::MIN + 1;
    let mut best_move: Option<EncodedMove> = None;

    let mut moves = board.get_legal_moves(false);

    // returns the mate score (very low) when in check but subtracts the depth to give a later check a better eval because the depth is lowers the further you go
    if moves.is_empty() {
        if board.is_in_check() {
            return (None, -MATE_SCORE - (depth as i32));
        } else {
            return (None, 0);
        }
    }

    move_ordering::order_moves(&mut moves, board);

    for mv in moves {
        let mut bc = board.clone();
        bc.make_move(&mv.decode());
        let eval = -alpha_beta(&bc, depth - 1, -beta, -alpha, stop, search_info).1;
        if eval > best_eval {
            best_eval = eval;
            best_move = Some(mv);
            if eval > alpha {
                alpha = eval;
            }
        }
        if eval >= beta {
            return (best_move, best_eval);
        }
    }

    (best_move, best_eval)
}

pub fn quiescence_search(
    board: &Board,
    mut alpha: i32,
    beta: i32,
    depth: usize,
    stop: &Arc<AtomicBool>,
    search_info: &mut SearchInfo,
) -> i32 {
    search_info.total_nodes += 1;

    if search_info.total_nodes % 1024 == 0 && stop.load(Ordering::Relaxed) {
        search_info.timeout_occurred = true;
        return 0;
    }

    if depth == 0 {
        return board.evaluate();
    }

    let stand_pat_score = board.evaluate();
    let mut best_score = stand_pat_score;

    if stand_pat_score >= beta {
        return stand_pat_score;
    }

    if alpha < stand_pat_score {
        alpha = stand_pat_score;
    }

    let mut capture_moves = board.get_legal_moves(true);
    move_ordering::order_moves(&mut capture_moves, board);

    for mv in capture_moves {
        let mut bc = board.clone();
        bc.make_move(&mv.decode());
        let score = -quiescence_search(board, -beta, -alpha, depth - 1, stop, search_info);

        if score >= beta {
            return score;
        }
        if score > best_score {
            best_score = score;
        }
        if score > alpha {
            alpha = score
        }
    }

    best_score
}
