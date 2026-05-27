//! Quiescence search utilities used to verify quiet training positions.
//!
//! During training data preparation, the engine performs a quiescence search from
//! the supplied root position and returns the final quiescent FEN of the best
//! line. This guarantees that training positions in the prepared file are quiet
//! with respect to the engine's capture and check-search mechanism.

use thunfisch::{
    evaluation::MATE_SCORE,
    move_picker::MovePicker,
    prelude::*,
    settings,
};

use std::sync::atomic::Ordering;

/// The result of a quiescence search.
///
/// `score` is the evaluated centipawn value of the best quiet position and
/// `best_line_fen` is the FEN of the best line returned by the search.
pub struct QuiescenceResult {
    pub score: i32,
    pub best_line_fen: String,
}

/// Perform a quiescence search and return the quietest line's final FEN.
///
/// The search explores capture sequences and check evasions only, and uses alpha-
/// beta pruning. The returned FEN is the board state reached at the terminal
/// leaf of the best quiescence path.
/// Note that we evaluate here using the main project's evaluation method.
#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
pub fn quiescence_search(
    depth: usize,
    mut alpha: i32,
    beta: i32,
    sd: &mut SharedSearchData,
    ply: usize,
) -> QuiescenceResult {
    *sd.local_seldepth = (*sd.local_seldepth).max(ply);

    if sd.board.is_threefold_repetition() || sd.board.is_50_move_rule() {
        return QuiescenceResult {
            score: 0,
            best_line_fen: sd.board.fen(),
        };
    }

    if depth == 0 {
        return QuiescenceResult {
            score: sd.board.evaluate(),
            best_line_fen: sd.board.fen(),
        };
    }

    sd.total_qs_nodes.fetch_add(1, Ordering::Relaxed);

    let is_check = sd.board.is_in_check();
    let eval = if is_check {
        -MATE_SCORE
    } else {
        sd.board.evaluate()
    };

    if eval >= beta {
        return QuiescenceResult {
            score: eval,
            best_line_fen: sd.board.fen(),
        };
    }

    if alpha < eval {
        alpha = eval;
    }

    let mut best_score = eval;
    let mut best_line_fen = sd.board.fen();
    let mut moves_searched = 0;

    let mut movepicker = if sd.board.is_in_check()
        && (ply - sd.ab_ply) < settings::QS_CHECK_EVASION_LIMIT
    {
        MovePicker::new(None, None, false)
    } else {
        MovePicker::new(None, None, true)
    };

    while let Some(mv) = movepicker.next(sd.board) {
        moves_searched += 1;

        sd.board.make_move(mv);
        let child = quiescence_search(depth - 1, -beta, -alpha, sd, ply + 1);
        sd.board.unmake_move();

        let score = -child.score;
        if score > best_score {
            best_score = score;
            best_line_fen = child.best_line_fen;
            if score > alpha {
                alpha = score;
            }

            if settings::AB && alpha >= beta {
                break;
            }
        }
    }

    if is_check && moves_searched == 0 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        return QuiescenceResult {
            score: -MATE_SCORE,
            best_line_fen: sd.board.fen(),
        };
    }

    QuiescenceResult {
        score: best_score,
        best_line_fen,
    }
}
