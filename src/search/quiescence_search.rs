use crate::prelude::*;
use crate::search::alpha_beta::MATE_SCORE;
use crate::search::move_picker::MovePicker;
use crate::search::transposition_table::Bound;
use crate::search::transposition_table::TT;
use crate::settings::settings;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// <https://www.chessprogramming.org/Quiescence_Search>
pub fn quiescence_search(
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    depth: usize,
    stop: &Arc<AtomicBool>,
    search_info: &SearchInfo,
    ply: usize,
    local_seldepth: &mut usize,
    ab_ply: usize,
) -> i32 {
    *local_seldepth = (*local_seldepth).max(ply);

    if stop.load(Ordering::Relaxed) {
        search_info.timeout_occurred.store(true, Ordering::Relaxed);
        return 0;
    }

    if board.is_threefold_repetition() || board.is_50_move_rule() {
        return 0;
    }

    if depth == 0 {
        return board.evaluate();
    }

    search_info.total_qs_nodes.fetch_add(1, Ordering::Relaxed);

    let original_alpha = alpha;
    let mut tt_move: Option<EncodedMove> = None;

    // Choose correct evaluation
    // Use the MATE eval if in check
    let is_check = board.is_in_check();
    let eval = if is_check {
        -MATE_SCORE
    } else if settings::TT_QS {
        // probe tt
        if let Some(tt_hit) = TT.probe(board.hash(), ply as i32) {
            search_info.total_tt_hits.fetch_add(1, Ordering::Relaxed);

            let tt_score = tt_hit.score();
            let bound = tt_hit.bound();

            tt_move = tt_hit.best_move();

            // Handle tt cuttoffs
            if match bound {
                Bound::Lower => tt_score >= beta,
                Bound::Upper => tt_score <= alpha,
                Bound::Exact => true,
                Bound::None => false,
            } {
                return tt_score;
            }

            let static_eval = board.evaluate();
            // Use TT score as eval when it refines the static eval:
            // - Upper (score <= tt): if tt < static, the position is worse than eval suggests
            // - Lower (score >= tt): if tt > static, the position is better than eval suggests
            // Otherwise fall back to static eval, as the bound doesn't contradict it.
            match tt_hit.bound() {
                Bound::Upper if tt_score < static_eval => tt_score,
                Bound::Lower if tt_score > static_eval => tt_score,
                _ => static_eval,
            }
        } else {
            // need normal eval when no tt hit
            board.evaluate()
        }
    } else {
        // ofc need normal eval when tt is completly disabled aswell
        board.evaluate()
    };

    if eval >= beta {
        if settings::TT_QS {
            TT.store(board.hash(), None, eval, 0, ply as i32, Bound::Lower, false);
        }
        return eval;
    }

    if alpha < eval {
        alpha = eval;
    }

    let mut best_score = eval;
    let mut best_move: Option<EncodedMove> = None;

    let mut i = 0;

    let mut movepicker = if board.is_in_check() && (ply - ab_ply) < settings::QS_CHECK_EVASION_LIMIT
    {
        MovePicker::new(tt_move, None, false)
    } else {
        MovePicker::new(tt_move, None, true)
    };

    // let initial_hash = board.hash();
    while let Some(mv) = movepicker.next(board) {
        i += 1;

        if stop.load(Ordering::Relaxed) {
            search_info.timeout_occurred.store(true, Ordering::Relaxed);
            return 0;
        }
        board.make_move(mv);
        let score = -quiescence_search(
            board,
            -beta,
            -alpha,
            depth - 1,
            stop,
            search_info,
            ply + 1,
            local_seldepth,
            ab_ply,
        );
        board.unmake_move();

        if score > best_score {
            best_score = score;
            if score > alpha {
                best_move = Some(mv);
                alpha = score;
            }

            if settings::AB
                && alpha >= beta {
                    break;
                }
        }
    }

    if is_check && i == 0 {
        return -MATE_SCORE + ply as i32;
    }

    let bound = if best_score >= beta {
        Bound::Lower
    } else if best_score > original_alpha {
        Bound::Exact
    } else {
        Bound::Upper
    };

    TT.store(
        board.hash(),
        best_move,
        best_score,
        0,
        ply as i32,
        bound,
        false,
    );

    best_score
}
