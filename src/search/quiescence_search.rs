use crate::prelude::*;
use crate::search::move_ordering;
use crate::search::search_info::SearchInfo;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

pub fn quiescence_search(
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    depth: usize,
    stop: &Arc<AtomicBool>,
    search_info: &mut SearchInfo,
) -> i32 {
    search_info.total_qs_nodes += 1;

    if search_info.total_alpha_beta_nodes % 32768 == 0 && stop.load(Ordering::Relaxed) {
        search_info.timeout_occurred = true;
        return 0;
    }

    if depth == 0 {
        return board.evaluate();
    }

    let stand_pat_score = board.evaluate();
    let mut best_score = stand_pat_score;

    if stand_pat_score >= beta {
        return stand_pat_score;
    }

    if alpha < stand_pat_score {
        alpha = stand_pat_score;
    }

    let mut moves: Vec<EncodedMove> = if board.is_in_check() {
        board.generate_moves(false)
    } else {
        board.generate_moves(true)
    };

    move_ordering::order_moves(&mut moves, board);

    for mv in moves {
        board.make_move(&mv.decode());
        let score = -quiescence_search(board, -beta, -alpha, depth - 1, stop, search_info);
        board.unmake_move();

        if score >= beta {
            return score;
        }
        if score > best_score {
            best_score = score;
        }
        if score > alpha {
            alpha = score
        }
    }

    best_score
}
