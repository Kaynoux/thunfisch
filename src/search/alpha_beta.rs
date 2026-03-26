use super::move_ordering;
use super::transposition_table::{TT, TTInfo};
use crate::prelude::*;
use crate::search::quiescence_search;
use crate::search::transposition_table::Bound;
use crate::settings::settings;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

pub const MATE_SCORE: i32 = 30_000;
const MAX_QUIESCENCE_SEARCH_DEPTH: usize = 12;

/// https://www.chessprogramming.org/Alpha-Beta
/// Returns the pv, and the associated evaluation
/// Note that the pv is reversed, i.e. the best move at this depth is at the end of the list
pub fn alpha_beta(
    board: &mut Board,
    depth: usize,
    mut alpha: i32,
    beta: i32,
    stop: &Arc<AtomicBool>,
    search_info: &SearchInfo,
    ply: usize,
    local_seldepth: &mut usize,
    null_move_allowed: bool,
) -> (Vec<EncodedMove>, i32) {
    *local_seldepth = (*local_seldepth).max(ply);
    search_info
        .total_alpha_beta_nodes
        .fetch_add(1, Ordering::Relaxed);

    if board.is_threefold_repetition() || board.is_50_move_rule() {
        return (vec![], 0);
    }

    let original_alpha = alpha;

    //let mut eval = board.evaluate(); // CONSIDER PERFORMANCE: not yet used but important for pruning (for example null move pruning)
    let mut tt_move: Option<EncodedMove> = None;
    let mut tt_score = i32::MIN + 1;

    // TODO: Implement Proper PVS if you want to use pv_node checks
    //let pv_node = beta > alpha + 1; // TODO

    if settings::TT_AB {
        // TODO: legal detection to prevent collisions
        if let Some(tt_hit) = TT.probe(board.hash(), ply as i32) {
            search_info.total_tt_hits.fetch_add(1, Ordering::Relaxed);

            let bound = tt_hit.bound();
            // let depth_cond = tt_hit.depth() >= depth as i32 - 3;

            tt_move = tt_hit.best_move();
            tt_score = tt_hit.score();

            // TODO: When using pvs add !pv_node as additionell condition
            let depth_req = depth as i32 + i32::from(tt_score >= beta);

            if settings::TT_CUTTOFFS {
                if tt_hit.depth() >= depth_req as i32
                    && match bound {
                        Bound::Lower => tt_score >= beta,
                        Bound::Upper => tt_score <= alpha,
                        Bound::Exact => true,
                        _ => false,
                    }
                {
                    return (vec![], tt_score);
                }
            }
        }
    }

    if depth == 0 {
        if settings::QS {
            let qs_result = quiescence_search::quiescence_search(
                board,
                alpha,
                beta,
                MAX_QUIESCENCE_SEARCH_DEPTH,
                stop,
                search_info,
                ply,
                local_seldepth,
                ply,
            );

            return (vec![], qs_result);
        }
        return (vec![], board.evaluate());
    };

    // cancels search if time is over
    if stop.load(Ordering::Relaxed) {
        search_info.timeout_occurred.store(true, Ordering::Relaxed);
        return (vec![], 0);
    }

    // Null move pruning
    // Do this before move generation to avoid costs induced by that
    if null_move_allowed && !board.is_in_check() {
        board.make_null_move();
        let reduction = 4 + depth / 6; // stolen from smol.cs
        let (_, eval) = alpha_beta(
            board,
            depth - reduction,
            -beta,
            -alpha,
            stop,
            search_info,
            ply + 1,
            local_seldepth,
            false,
        );
        if eval >= beta {
            return (vec![], beta);
        }
    }

    // set the best evaluation very low to begin with
    let mut best_eval = i32::MIN + 1;
    let mut best_move: Option<EncodedMove> = None;

    let mut moves = board.generate_moves::<false>();

    // returns the mate score (very low) when in check but adds the ply to give a later check a better eval because the depth is lowers the further you go
    if moves.is_empty() {
        if board.is_in_check() {
            return (vec![], -MATE_SCORE + (ply as i32));
        } else {
            return (vec![], 0);
        }
    }

    if settings::MVV_LVA {
        move_ordering::order_moves(&mut moves, board, tt_move);
    }

    // Probe again as there's a chance a different thread updated the TT since we last probed it
    // Also probe with maximum possible window to ensure we get an entry if it exists
    // if let Some((_, tt_mv)) = TT.probe(hash, i32::MAX, i32::MIN, 0) {
    //     if let Some(pos) = moves.iter().position(|&m| m == tt_mv) {
    //         moves.swap(0, pos);
    //     }
    // }
    // TODO: Figure out whether that is actually needed

    let mut pv = vec![];
    for mv in moves {
        // cancels search if time is over
        if stop.load(Ordering::Relaxed) {
            search_info.timeout_occurred.store(true, Ordering::Relaxed);
            return (vec![], 0);
        }
        board.make_move(&mv.decode());
        let (mut local_pv, mut eval) = alpha_beta(
            board,
            depth - 1,
            -beta,
            -alpha,
            stop,
            search_info,
            ply + 1,
            local_seldepth,
            true,
        );
        // do the negamax negation here since we can't negate a tuple above
        eval *= -1;
        board.unmake_move();

        if eval > best_eval {
            best_eval = eval;
            if eval > alpha {
                best_move = Some(mv);
                alpha = eval
            }

            if settings::AB {
                if alpha >= beta {
                    break;
                }
            }
        }
    }

    let bound = if best_eval >= beta {
        Bound::Lower
    } else if best_eval > original_alpha {
        Bound::Exact
    } else {
        Bound::Upper
    };

    if settings::TT_AB {
        TT.store(
            board.hash(),
            best_move,
            best_eval,
            depth as i8,
            ply as i32,
            bound,
            false,
        );
    }

    (pv, best_eval)
}
