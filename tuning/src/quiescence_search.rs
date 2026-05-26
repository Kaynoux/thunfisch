use thunfisch::{
    evaluation::MATE_SCORE,
    move_picker::MovePicker,
    prelude::*,
    settings,
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

    // Choose correct evaluation
    // Use the MATE eval if in check
    let is_check = sd.board.is_in_check();
    let eval = if is_check {
        -MATE_SCORE
    } else {
        // ofc need normal eval when tt is completly disabled aswell
        sd.board.evaluate()
    };

    if eval >= beta {
        return eval;
    }

    if alpha < eval {
        alpha = eval;
    }

    let mut best_score = eval;

    let mut i = 0;

    let mut movepicker =
        if sd.board.is_in_check() && (ply - sd.ab_ply) < settings::QS_CHECK_EVASION_LIMIT {
            MovePicker::new(None, None, false)
        } else {
            MovePicker::new(None, None, true)
        };

    // let initial_hash = board.hash();
    while let Some(mv) = movepicker.next(sd.board) {
        i += 1;

        if sd.stop.load(Ordering::Relaxed) {
            sd.timeout_occurred.store(true, Ordering::Relaxed);
            return 0;
        }
        sd.board.make_move(mv);
        let score = -quiescence_search(depth - 1, -beta, -alpha, sd, ply + 1);
        sd.board.unmake_move();

        if score > best_score {
            best_score = score;
            if score > alpha {
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

    best_score
}
