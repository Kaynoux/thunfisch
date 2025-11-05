use crate::prelude::*;
use crate::search::move_ordering;
use crate::search::transposition_table::ScoreType;
use crate::settings::settings;
use crate::{move_generator::generator::ARRAY_LENGTH, search::transposition_table::TT};
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

    if board.is_threefold_repetition() || board.is_50_move_rule() {
        return 0;
    }

    let hash = board.hash();
    if settings::TRANSPOSITION_TABLE {
        if let Some(tt_hit) = TT.probe(hash, alpha, beta, depth) {
            search_info.total_tt_hits.fetch_add(1, Ordering::Relaxed);
            return tt_hit.0;
        }
    }

    if depth == 0 {
        let eval = board.evaluate();
        TT.store(hash, None, eval, depth, ScoreType::Exact);
        return eval;
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

    if settings::MOVE_ORDERING {
        move_ordering::order_moves(&mut moves, board);
    }

    let mut score_type = ScoreType::UpperBound;
    let mut best_mv = None;
    for mv in moves {
        if stop.load(Ordering::Relaxed) {
            search_info.timeout_occurred.store(true, Ordering::Relaxed);
            return 0;
        }
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

        if settings::ALPHA_BETA {
            if score >= beta {
                TT.store(hash, Some(mv), score, depth, ScoreType::LowerBound);
                return score;
            }
        }

        if score > best_score {
            best_score = score;
            best_mv = Some(mv);
        }
        if score > alpha {
            alpha = score;
            score_type = ScoreType::Exact;
        }
    }

    TT.store(hash, best_mv, alpha, depth, score_type);

    best_score
}
