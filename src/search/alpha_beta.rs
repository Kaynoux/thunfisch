use super::move_ordering;
use super::transposition_table::TT;
use crate::prelude::*;
use crate::search::quiescence_search;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

const MATE_SCORE: i32 = 100000;
const MAX_QUIESCENCE_SEARCH_DEPTH: usize = 12;

/// https://www.chessprogramming.org/Alpha-Beta
pub fn alpha_beta(
    board: &mut Board,
    depth: usize,
    mut alpha: i32,
    beta: i32,
    stop: &Arc<AtomicBool>,
    search_info: &SearchInfo,
    ply: usize,
    local_seldepth: &mut usize,
) -> (Option<EncodedMove>, i32) {
    *local_seldepth = (*local_seldepth).max(ply);

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
                ply,
                local_seldepth,
            ),
        );
    }

    search_info
        .total_alpha_beta_nodes
        .fetch_add(1, Ordering::Relaxed);

    // cancels search if time is over
    if stop.load(Ordering::Relaxed) {
        search_info.timeout_occurred.store(true, Ordering::Relaxed);
        return (None, 0);
    }

    // set the best evaluation very low to begin with
    let mut best_eval = i32::MIN + 1;
    let mut best_move: Option<EncodedMove> = None;

    let mut moves = board.generate_moves::<false>();

    // returns the mate score (very low) when in check but subtracts the depth to give a later check a better eval because the depth is lowers the further you go
    if moves.is_empty() {
        if board.is_in_check() {
            return (None, -MATE_SCORE - (depth as i32));
        } else {
            return (None, 0);
        }
    }

    move_ordering::order_moves(&mut moves, board);

    let hash = board.hash();
    if let Some(tt_mv) = TT.probe(hash) {
        if let Some(pos) = moves.iter().position(|&m| m == tt_mv) {
            moves.swap(0, pos);
        }
    }

    for mv in moves {
        // cancels search if time is over
        if stop.load(Ordering::Relaxed) {
            search_info.timeout_occurred.store(true, Ordering::Relaxed);
            return (None, 0);
        }
        board.make_move(&mv.decode());
        let eval = -alpha_beta(
            board,
            depth - 1,
            -beta,
            -alpha,
            stop,
            search_info,
            ply + 1,
            local_seldepth,
        )
        .1;
        board.unmake_move();

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

    if let Some(mv) = best_move {
        TT.store(hash, mv);
    }

    (best_move, best_eval)
}
