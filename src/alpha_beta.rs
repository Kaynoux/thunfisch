use crate::{
    evaluation::MATE_SCORE,
    move_picker::MovePicker,
    prelude::*,
    quiescence_search,
    settings::{self, MAX_AB_DEPTH, RFP_MARGIN},
    transposition_table::{Bound, TT},
};

use std::{
    cmp::min,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

/// There's different approaches to this one as well. CPW suggests 150 * depth, smol.cs does 75 * depth.
/// Generally: The smaller `rfp_margin`, the more aggressively we prune.
/// THIS IS TUNABLE.
#[inline]
pub const fn rfp_margin(depth: usize) -> usize {
    RFP_MARGIN * depth
}

/// <https://www.chessprogramming.org/Alpha-Beta>
/// Returns the pv, and the associated evaluation
/// Note that the pv is reversed, i.e. the best move at this depth is at the end of the list
/// TODO: Reduce number of arguments into a struct
#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::needless_pass_by_value // because node_type should be pass by value because it is a bool essentially 
)]
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
    node_type: NodeType,
    killers: &mut [EncodedMove; MAX_AB_DEPTH],
) -> i32 {
    *local_seldepth = (*local_seldepth).max(ply);
    search_info
        .total_alpha_beta_nodes
        .fetch_add(1, Ordering::Relaxed);

    if board.is_threefold_repetition() || board.is_50_move_rule() {
        return 0;
    }

    if depth == 0 {
        if settings::QS && ply < MAX_AB_DEPTH - 1 {
            let qs_result = quiescence_search::quiescence_search(
                board,
                alpha,
                beta,
                settings::MAX_QS_DEPTH,
                stop,
                search_info,
                ply,
                local_seldepth,
                ply,
            );

            return qs_result;
        }
        return board.evaluate();
    }

    let original_alpha = alpha;
    let eval = board.evaluate();

    let mut tt_move: Option<EncodedMove> = None;
    let tt_score;

    // TODO: Implement Proper PVS if you want to use pv_node checks
    //let pv_node = beta > alpha + 1; // TODO

    if settings::TT_AB {
        // TODO: legal detection to prevent collisions
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        if let Some(tt_hit) = TT.probe(board.hash(), ply as i32) {
            search_info.total_tt_hits.fetch_add(1, Ordering::Relaxed);

            let bound = tt_hit.bound();
            // let depth_cond = tt_hit.depth() >= depth as i32 - 3;

            tt_move = tt_hit.best_move();
            tt_score = tt_hit.score();

            // TODO: When using pvs add !pv_node as additionell condition
            let depth_req = depth as i32 + i32::from(tt_score >= beta);

            if settings::TT_CUTTOFFS
                && (node_type != NodeType::OnPV || !settings::PVS)
                && tt_hit.depth() >= depth_req
                && match bound {
                    Bound::Lower => tt_score >= beta,
                    Bound::Upper => tt_score <= alpha,
                    Bound::Exact => true,
                    Bound::None => false,
                }
            {
                return tt_score;
            }
        }
    }

    // cancels search if time is over
    if stop.load(Ordering::Relaxed) {
        search_info.timeout_occurred.store(true, Ordering::Relaxed);
        return 0;
    }

    if node_type != NodeType::OnPV || !settings::PVS {
        if settings::RFP {
            // apparently RFP should only be done in the later parts of the tree. CPW explicitly mentions
            // pre-frontier nodes, i.e. those nodes where depth == 1. However viridithias, smol.cs and akimbo
            // use higher values, so this is likely tunable. Stockfish from what I can tell does depth < 2.
            //
            // NOTE ON THIS: Higher values for depth limiting crash Thunfisch into oblivion.
            // If we get the branching factor under control and consistently hit depth 13-15 we can probably bump this up to ~4
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            if depth < 4
                && !board.is_in_check()
                && eval >= beta
                && eval >= beta + rfp_margin(depth) as i32
            {
                return eval;
            }
        }

        // Do this before move generation to avoid generation costs
        if settings::NMP
            && null_move_allowed
            && !board.is_in_check()
            && !board.is_king_pawn_endgame()
            && eval >= beta
        {
            board.make_null_move();
            let reduction = min(depth, 4);
            let eval = -alpha_beta(
                board,
                depth - reduction,
                -alpha - 1,
                -alpha,
                stop,
                search_info,
                ply + 1,
                local_seldepth,
                false,
                NodeType::OffPV,
                killers,
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

    let killer_mv: Option<EncodedMove> = killers
        .get(ply)
        .and_then(|&mv| if mv == EncodedMove(0) { None } else { Some(mv) });

    let mut movepicker = MovePicker::new(tt_move, killer_mv, false);
    let mut i = 0;

    // let initial_hash = board.hash();
    while let Some(mv) = movepicker.next(board) {
        i += 1;
        // cancels search if time is over
        if stop.load(Ordering::Relaxed) {
            search_info.timeout_occurred.store(true, Ordering::Relaxed);
            return 0;
        }
        board.make_move(mv);
        let mut eval;
        if i == 1 || !settings::PVS {
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
                NodeType::OnPV,
                killers,
            );
        } else {
            // Search non-PV moves with null window
            eval = -alpha_beta(
                board,
                depth - 1,
                -alpha - 1,
                -alpha,
                stop,
                search_info,
                ply + 1,
                local_seldepth,
                true,
                NodeType::OffPV,
                killers,
            );
            // Re-search if non-PV move raised Alpha
            // ONLY do re-searching if the current Node is on PV - we don't care for re-searching OffPV nodes
            // (if they actually improve over the PV that variation will be searched again at the last PV node anyway)
            if eval > alpha && node_type == NodeType::OnPV {
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
                    NodeType::OnPV,
                    killers,
                );
            }
        }

        board.unmake_move();

        if eval > best_eval {
            best_eval = eval;
            if eval > alpha {
                best_move = Some(mv);
                alpha = eval;
            }

            if settings::AB && alpha >= beta {
                if mv.decode().is_quiet() {
                    killers[ply] = mv;
                }
                break;
            }
        }
    }

    // When i still 0 than no move found
    // returns the mate score (very low) when in check but adds the ply to give a later check a better eval because the depth is lowers the further you go
    if i == 0 {
        if board.is_in_check() {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            return MATE_SCORE + (ply as i32);
        }
        return 0;
    }

    let bound = if best_eval >= beta {
        Bound::Lower
    } else if best_eval > original_alpha {
        Bound::Exact
    } else {
        Bound::Upper
    };

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
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
