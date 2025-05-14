use crate::move_generator::generator::ARRAY_LENGTH;
use crate::prelude::*;
use crate::search::move_ordering;
use arrayvec::ArrayVec;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// https://www.chessprogramming.org/Quiescence_Search
pub fn quiescence_search(
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    depth: usize,
    stop: &Arc<AtomicBool>,
    search_info: &SearchInfo,
    ply: usize,
    local_seldepth: &mut usize,
) -> i32 {
    *local_seldepth = (*local_seldepth).max(ply);

    if stop.load(Ordering::Relaxed) {
        search_info.timeout_occurred.store(true, Ordering::Relaxed);
        return 0;
    }

    if depth == 0 {
        return board.evaluate();
    }

    search_info.total_qs_nodes.fetch_add(1, Ordering::Relaxed);

    let stand_pat_score = board.evaluate();
    let mut best_score = stand_pat_score;

    if stand_pat_score >= beta {
        return stand_pat_score;
    }

    if alpha < stand_pat_score {
        alpha = stand_pat_score;
    }

    let mut moves: ArrayVec<EncodedMove, ARRAY_LENGTH> = if board.is_in_check() {
        board.generate_moves::<false>()
    } else {
        board.generate_moves::<true>()
    };

    move_ordering::order_moves(&mut moves, board);

    for mv in moves {
        board.make_move(&mv.decode());
        let score = -quiescence_search(
            board,
            -beta,
            -alpha,
            depth - 1,
            stop,
            search_info,
            ply + 1,
            local_seldepth,
        );
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
