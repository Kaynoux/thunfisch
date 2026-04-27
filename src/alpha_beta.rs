use arrayvec::ArrayVec;

use crate::{
    evaluation::MATE_SCORE,
    move_picker::MovePicker,
    move_scoring::{HISTORY_TABLE, history_bonus, history_maluse},
    prelude::*,
    quiescence_search,
    settings::{self, MAX_AB_DEPTH, RFP_MARGIN},
    transposition_table::{Bound, TT},
};

use std::{cmp::min, f64, sync::atomic::Ordering};

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
///
/// Shared data is extracted to a struct, to keep the number of arguments <= 6. This should, in theory,
/// optimize the function as all arguments can be passed through registers
#[allow(clippy::too_many_lines)]
pub fn alpha_beta<const PV_NODE: bool>(
    depth: usize,
    mut alpha: i32,
    beta: i32,
    sd: &mut SharedSearchData,
    ply: usize,
    null_move_allowed: bool,
) -> i32 {
    *sd.local_seldepth = (*sd.local_seldepth).max(ply);
    sd.total_alpha_beta_nodes.fetch_add(1, Ordering::Relaxed);

    assert!(depth <= MAX_AB_DEPTH);

    if sd.board.is_threefold_repetition() || sd.board.is_50_move_rule() {
        return 0;
    }

    if depth == 0 {
        if settings::QS && ply < MAX_AB_DEPTH - 1 {
            sd.ab_ply = ply;
            let qs_result =
                quiescence_search::quiescence_search(settings::MAX_QS_DEPTH, alpha, beta, sd, ply);

            return qs_result;
        }
        return sd.board.evaluate();
    }

    let original_alpha = alpha;
    let eval = sd.board.evaluate();

    let mut tt_move: Option<EncodedMove> = None;
    let tt_score;

    if settings::TT_AB {
        // TODO: legal detection to prevent collisions
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        if let Some(tt_hit) = TT.probe(sd.board.hash(), ply as i32) {
            sd.total_tt_hits.fetch_add(1, Ordering::Relaxed);

            let bound = tt_hit.bound();

            tt_move = tt_hit.best_move();
            tt_score = tt_hit.score();

            let depth_req = depth as i32 + i32::from(tt_score >= beta);

            if settings::TT_CUTTOFFS
                && (!PV_NODE || !settings::PVS)
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
    if sd.stop.load(Ordering::Relaxed) {
        sd.timeout_occurred.store(true, Ordering::Relaxed);
        return 0;
    }

    if !PV_NODE || !settings::PVS {
        if settings::RFP {
            // apparently RFP should only be done in the later parts of the tree. CPW explicitly mentions
            // pre-frontier nodes, i.e. those nodes where depth == 1. However viridithias, smol.cs and akimbo
            // use higher values, so this is likely tunable. Stockfish from what I can tell does depth < 2.
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            if depth < 4
                && !sd.board.is_in_check()
                && eval >= beta
                && eval >= beta + rfp_margin(depth) as i32
            {
                return eval;
            }
        }

        // Do this before move generation to avoid generation costs
        if settings::NMP
            && null_move_allowed
            && !sd.board.is_in_check()
            && !sd.board.is_king_pawn_endgame()
            && eval >= beta
        {
            sd.board.make_null_move();
            let reduction = min(depth, 4);
            let eval =
                -alpha_beta::<false>(depth - reduction, -alpha - 1, -alpha, sd, ply + 1, false);
            sd.board.unmake_null_move();
            if eval >= beta {
                return beta;
            }
        }
    }

    // set the best evaluation very low to begin with
    let mut best_eval = i32::MIN + 1;
    let mut best_move: Option<EncodedMove> = None;

    let killer_mv: Option<EncodedMove> = sd
        .killers
        .get(ply)
        .and_then(|&mv| if mv == EncodedMove(0) { None } else { Some(mv) });

    let mut quiets_tried: ArrayVec<EncodedMove, 256> = ArrayVec::new();
    let mut movepicker = MovePicker::new(tt_move, killer_mv, false);
    let mut moves_visited = 0;

    while let Some(mv) = movepicker.next(sd.board) {
        moves_visited += 1;
        // cancels search if time is over
        if sd.stop.load(Ordering::Relaxed) {
            sd.timeout_occurred.store(true, Ordering::Relaxed);
            return 0;
        }
        sd.board.make_move(mv);
        let mut eval;
        if moves_visited == 1 || !settings::PVS {
            // Principal Variation Search
            // We assume that the first move from the move ordering is the PV move;
            // Since the TT move, if existant, is in first place anyway this automatically includes information from shallower search depths
            eval = -alpha_beta::<true>(depth - 1, -beta, -alpha, sd, ply + 1, true);
        } else {
            let mut reduction = 1;

            #[allow(clippy::useless_let_if_seq, clippy::cast_possible_truncation)]
            if settings::LMR && moves_visited >= settings::MOVES_BEFORE_LMR && depth > 2 {
                // ensure we always reduce less than `depth`, otherwise we run into overflows and search until the end of the universe
                reduction +=
                    LMR_REDUCTION[depth.clamp(0, 63)][moves_visited.clamp(0, 63)].min(depth as u32);
            }

            debug_assert!(
                reduction as usize <= depth,
                "have fun at the end of the universe"
            );

            eval = -alpha_beta::<false>(
                depth - reduction as usize,
                -alpha - 1,
                -alpha,
                sd,
                ply + 1,
                true,
            );

            if eval > alpha {
                // If our shallow-depth search raised alpha, we perform a search at full depth but still with a null window
                // in hopes that we can still avoid a full search
                if reduction > 1 {
                    // Search non-PV moves with null window
                    sd.total_lmr_researches.fetch_add(1, Ordering::Relaxed);
                    eval = -alpha_beta::<false>(depth - 1, -alpha - 1, -alpha, sd, ply + 1, true);
                }
                // ONLY do full-window full-depth re-searching if the current Node is on PV - we don't care for re-searching OffPV nodes
                // (if they actually improve over the PV that variation will be searched again at the last PV node anyway)
                if eval > alpha && PV_NODE {
                    sd.total_pvs_researches.fetch_add(1, Ordering::Relaxed);
                    eval = -alpha_beta::<true>(depth - 1, -beta, -alpha, sd, ply + 1, true);
                }
            }
        }

        sd.board.unmake_move();

        if eval > best_eval {
            best_eval = eval;
            if eval > alpha {
                best_move = Some(mv);
                alpha = eval;
            }

            if settings::AB && alpha >= beta {
                if mv.decode().is_quiet() {
                    sd.killers[ply] = mv;
                }
                break;
            }
        }
        // TODO try whether it gains to punish ALL quiet moves (including TT and killers)
        // instead of just the one generated
        if mv.decode().is_quiet() {
            quiets_tried.push(mv);
        }
    }

    // When i still 0 than no move found
    // returns the mate score (very low) when in check but adds the ply to give a later check a better eval because the depth is lowers the further you go
    if moves_visited == 0 {
        if sd.board.is_in_check() {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            return -MATE_SCORE + (ply as i32);
        }
        return 0;
    }

    // If the best move is quiet, adjust the history values
    if settings::HISTORIES
        && let Some(best_move) = best_move
        && best_move.decode().is_quiet()
    {
        // Give bonus to best move
        HISTORY_TABLE.update_history(
            sd.board.current_color(),
            best_move.decode(),
            history_bonus(depth),
        );

        // Punish quiet moves ordered before the best quiet move (maluse)
        for mv in quiets_tried {
            if mv == best_move {
                break;
            }
            HISTORY_TABLE.update_history(
                sd.board.current_color(),
                mv.decode(),
                history_maluse(depth),
            );
        }
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
        sd.board.hash(),
        best_move,
        best_eval,
        depth as i8,
        ply as i32,
        bound,
        false,
    );

    best_eval
}

// const LMP_LAZYLOCK = s

// store the base LMR reductions statically
const LMR_REDUCTION: [[u32; 64]; 64] = {
    let mut out = [[0u32; 64]; 64];

    let mut depth = 1;
    while depth < 64 {
        let mut moves_visited = 1;
        while moves_visited < 64 {
            out[depth][moves_visited] = lmr_reduction(depth, moves_visited);
            // assert!(lmr_reduction(depth, moves_visited) != 0, "we have a non-zero value");
            moves_visited += 1;
        }
        depth += 1;
    }
    out
};

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
const fn lmr_reduction(depth: usize, moves_visited: usize) -> u32 {
    (1.35 + int_ln(depth) as f64 * int_ln(moves_visited) as f64 / 2.75) as u32
}

/// Method really is only there because `f64::ln` is not a const function and so cannot be called at compile time
/// This here expresses a ln on integers using the constant log 2
#[allow(
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
const fn int_ln(n: usize) -> u32 {
    (1.0 / f64::consts::LOG2_E * n.ilog2() as f64) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_lmp_board() {
        println!("{}", u32::from(false));
        println!("{LMR_REDUCTION:?}");
    }
}
