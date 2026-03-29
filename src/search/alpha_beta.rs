use super::move_ordering;
use super::transposition_table::TT;
use crate::prelude::*;
use crate::search::quiescence_search;
use crate::search::transposition_table::Bound;
use crate::settings::settings;

use std::cmp::min;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

pub const MATE_SCORE: i32 = 30_000;
const MAX_QUIESCENCE_SEARCH_DEPTH: usize = 12;

/// There's different approaches to this one as well. CPW suggests 150 * depth, smol.cs does 75 * depth.
/// Generally: The smaller `rfp_margin`, the more aggressively we prune.
/// THIS IS TUNABLE.
#[inline(always)]
pub const fn rfp_margin(depth: usize) -> usize {
    75 * depth
}

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
    tt_cutoffs_allowed: bool,
    node_type: NodeType,
) -> i32 {
    *local_seldepth = (*local_seldepth).max(ply);
    search_info
        .total_alpha_beta_nodes
        .fetch_add(1, Ordering::Relaxed);

    if board.is_threefold_repetition() || board.is_50_move_rule() {
        return 0;
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

            return qs_result;
        }
        return board.evaluate();
    };

    let original_alpha = alpha;
    let eval = board.evaluate();

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

            if settings::TT_CUTTOFFS && tt_cutoffs_allowed {
                if tt_hit.depth() >= depth_req as i32
                    && match bound {
                        Bound::Lower => tt_score >= beta,
                        Bound::Upper => tt_score <= alpha,
                        Bound::Exact => true,
                        _ => false,
                    }
                {
                    return tt_score;
                }
            }
        }
    }

    // cancels search if time is over
    if stop.load(Ordering::Relaxed) {
        search_info.timeout_occurred.store(true, Ordering::Relaxed);
        return 0;
    }

    if settings::RFP {
        // apparently RFP should only be done in the later parts of the tree. CPW explicitly mentions
        // pre-frontier nodes, i.e. those nodes where depth == 1. However viridithias, smol.cs and akimbo
        // use higher values, so this is likely tunable. Stockfish from what I can tell does depth < 2.
        //
        // NOTE ON THIS: Higher values for depth limiting crash Thunfisch into oblivion.
        // If we get the branching factor under control and consistently hit depth 13-15 we can probably bump this up to ~4
        if depth < 2 && !board.is_in_check() && eval >= beta {
            if eval >= beta + rfp_margin(depth) as i32 {
                return eval;
            }
        }
    }

    // Do this before move generation to avoid generation costs
    if settings::NMP {
        if null_move_allowed
            && !board.is_in_check()
            && !board.is_king_pawn_endgame()
            && eval >= beta
        {
            board.make_null_move();
            let reduction = min(depth, 4);
            let eval = -alpha_beta(
                board,
                depth - reduction,
                -beta,
                -alpha,
                stop,
                search_info,
                ply + 1,
                local_seldepth,
                false,
                true,
                NodeType::OffPV,
            );
            board.unmake_null_move();
            if eval >= beta {
                return beta;
            }
        }
    }
    // set the best evaluation very low to begin with
    let mut best_eval = i32::MIN + 1;
    let mut best_move: Option<EncodedMove> = None;

    let mut moves = board.generate_moves::<false>();

    // returns the mate score (very low) when in check but adds the ply to give a later check a better eval because the depth is lowers the further you go
    if moves.is_empty() {
        if board.is_in_check() {
            return -MATE_SCORE + (ply as i32);
        } else {
            return 0;
        }
    }

    move_ordering::order_moves(&mut moves, board, tt_move);

    for (i, mv) in moves.iter().enumerate() {
        // cancels search if time is over
        if stop.load(Ordering::Relaxed) {
            search_info.timeout_occurred.store(true, Ordering::Relaxed);
            return 0;
        }
        board.make_move(&mv.decode());
        let mut eval;
        if i == 0 || !settings::PVS {
            // Principal Variation Search
            // We assume that the first move from the move ordering is the PV move;
            // Since the TT move, if existant, is in first place anyway this automatically includes information from shallower search depths

            eval = -alpha_beta(
                board,
                depth - 1,
                -beta,
                -alpha,
                stop,
                search_info,
                ply + 1,
                local_seldepth,
                true,
                board.count_repetitions() == 0,
                NodeType::OnPV,
            );
        } else {
            eval = -alpha_beta(
                board,
                depth - 1,
                -alpha - 1, // zero-window search for OffPV nodes
                -alpha,
                stop,
                search_info,
                ply + 1,
                local_seldepth,
                true,
                true,
                NodeType::OffPV,
            );
            // Re-search if OffPV-Node raised alpha
            if eval > alpha && node_type != NodeType::OnPV {
                eval = -alpha_beta(
                    board,
                    depth - 1,
                    -beta,
                    -alpha,
                    stop,
                    search_info,
                    ply + 1,
                    local_seldepth,
                    true,
                    board.count_repetitions() == 0,
                    NodeType::OnPV,
                );
            }
        }

        board.unmake_move();

        if eval > best_eval {
            best_eval = eval;
            if eval > alpha {
                best_move = Some(*mv);
                alpha = eval;
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

    TT.store(
        board.hash(),
        best_move,
        best_eval,
        depth as i8,
        ply as i32,
        bound,
        false,
    );

    best_eval
}
