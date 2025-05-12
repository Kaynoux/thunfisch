use super::move_ordering;
use crate::prelude::*;
use crate::search::quiescence_search;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

const MATE_SCORE: i32 = 100000;
const MAX_QUIESCENCE_SEARCH_DEPTH: usize = 4;

/// Modified Mini Max algorithm which always picks the best move for each side until a given depth is reached and the evaluates the outcomes to pick the best move at the first layer
pub fn alpha_beta(
    board: &mut Board,
    depth: usize,
    mut alpha: i32,
    beta: i32,
    stop: &Arc<AtomicBool>,
    search_info: &mut SearchInfo,
) -> (Option<EncodedMove>, i32) {
    search_info.total_alpha_beta_nodes += 1;
    if depth == 0 {
        return (
            None,
            quiescence_search::quiescence_search(
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
    if search_info.total_alpha_beta_nodes % 32768 == 0 && stop.load(Ordering::Relaxed) {
        search_info.timeout_occurred = true;
        return (None, 0);
    }

    // set the best evaluation very low to begin with
    let mut best_eval = i32::MIN + 1;
    let mut best_move: Option<EncodedMove> = None;

    let moves = board.generate_moves(false);

    // returns the mate score (very low) when in check but subtracts the depth to give a later check a better eval because the depth is lowers the further you go
    if moves.is_empty() {
        if board.is_in_check() {
            return (None, -MATE_SCORE - (depth as i32));
        } else {
            return (None, 0);
        }
    }

    //move_ordering::order_moves(&mut moves, board);

    for mv in moves {
        let mut bc = board.clone();
        //board.make_move(&mv.decode());
        let eval = -alpha_beta(&mut bc, depth - 1, -beta, -alpha, stop, search_info).1;
        //board.unmake_move();

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
