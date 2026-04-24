use crate::{
    evaluation::MATE_SCORE,
    move_picker::MovePicker,
    prelude::*,
    settings,
    transposition_table::{Bound, TT},
};

use std::sync::atomic::Ordering;

/// <https://www.chessprogramming.org/Quiescence_Search>
#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
pub fn quiescence_search(
    depth: usize,
    mut alpha: i32,
    beta: i32,
    sd: &mut SharedSearchData,
    ply: usize,
) -> i32 {
    *sd.local_seldepth = (*sd.local_seldepth).max(ply);

    if sd.stop.load(Ordering::Relaxed) {
        sd.timeout_occurred.store(true, Ordering::Relaxed);
        return 0;
    }

    if sd.board.is_threefold_repetition() || sd.board.is_50_move_rule() {
        return 0;
    }

    if depth == 0 {
        return sd.board.evaluate();
    }

    sd.total_qs_nodes.fetch_add(1, Ordering::Relaxed);

    let original_alpha = alpha;
    let mut tt_move: Option<EncodedMove> = None;

    // Choose correct evaluation
    // Use the MATE eval if in check
    let is_check = sd.board.is_in_check();
    let eval = if is_check {
        -MATE_SCORE
    } else if settings::TT_QS {
        // probe tt
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        if let Some(tt_hit) = TT.probe(sd.board.hash(), ply as i32) {
            sd.total_tt_hits.fetch_add(1, Ordering::Relaxed);

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

            let static_eval = sd.board.evaluate();
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
            sd.board.evaluate()
        }
    } else {
        // ofc need normal eval when tt is completly disabled aswell
        sd.board.evaluate()
    };

    if eval >= beta {
        if settings::TT_QS {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            TT.store(
                sd.board.hash(),
                None,
                eval,
                0,
                ply as i32,
                Bound::Lower,
                false,
            );
        }
        return eval;
    }

    if alpha < eval {
        alpha = eval;
    }

    let mut best_score = eval;
    let mut best_move: Option<EncodedMove> = None;

    let mut i = 0;

    let mut movepicker =
        if sd.board.is_in_check() && (ply - sd.ab_ply) < settings::QS_CHECK_EVASION_LIMIT {
            MovePicker::new(tt_move, None, false)
        } else {
            MovePicker::new(tt_move, None, true)
        };

    // let initial_hash = board.hash();
    while let Some(mv) = movepicker.next(sd.board) {
        i += 1;

        if sd.stop.load(Ordering::Relaxed) {
            sd.timeout_occurred.store(true, Ordering::Relaxed);
            return 0;
        }
        sd.board.make_move(mv);
        let score = -quiescence_search(depth - 1, -beta, -alpha, sd, ply);
        sd.board.unmake_move();

        if score > best_score {
            best_score = score;
            if score > alpha {
                best_move = Some(mv);
                alpha = score;
            }

            if settings::AB && alpha >= beta {
                break;
            }
        }
    }

    if is_check && i == 0 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        return -MATE_SCORE + ply as i32;
    }

    let bound = if best_score >= beta {
        Bound::Lower
    } else if best_score > original_alpha {
        Bound::Exact
    } else {
        Bound::Upper
    };

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    TT.store(
        sd.board.hash(),
        best_move,
        best_score,
        0,
        ply as i32,
        bound,
        false,
    );

    best_score
}
