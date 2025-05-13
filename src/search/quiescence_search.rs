use crate::move_generator::generator::ARRAY_LENGTH;
use crate::prelude::*;
use crate::search::{
    move_ordering,
    transposition_table::{TTFlag, TranspositionTable},
};
use arrayvec::ArrayVec;

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
    tt: &mut TranspositionTable,
) -> i32 {
    search_info.total_qs_nodes += 1;

    let key = board.hash();
    let alpha_orig = alpha;
    if depth > 0 {
        search_info.tt_probes += 1;
        if let Some(entry) = tt.probe(key, depth as u8) {
            search_info.tt_hits += 1;
            match entry.flag {
                TTFlag::Exact => return entry.eval,
                TTFlag::LowerBound => alpha = alpha.max(entry.eval),
                TTFlag::UpperBound => {
                    if entry.eval <= alpha {
                        return entry.eval;
                    }
                }
            }
        }
    }

    if search_info.total_alpha_beta_nodes % 32768 == 0 && stop.load(Ordering::Relaxed) {
        return 0;
    }

    if depth == 0 {
        let eval = board.evaluate();
        tt.store(key, 0, eval, TTFlag::Exact, None);
        return eval;
    }

    let stand_pat_score = board.evaluate();
    let mut best_score = stand_pat_score;

    if stand_pat_score >= beta {
        search_info.tt_stores += 1;
        tt.store(key, depth as u8, stand_pat_score, TTFlag::LowerBound, None);
        return stand_pat_score;
    }

    if alpha < stand_pat_score {
        alpha = stand_pat_score;
    }

    let mut moves: ArrayVec<EncodedMove, ARRAY_LENGTH> = if board.is_in_check() {
        board.generate_moves(false)
    } else {
        board.generate_moves(true)
    };

    move_ordering::order_moves(&mut moves, board);

    for mv in moves {
        board.make_move(&mv.decode());
        let score = -quiescence_search(board, -beta, -alpha, depth - 1, stop, search_info, tt);
        board.unmake_move();

        if score >= beta {
            search_info.tt_stores += 1;
            tt.store(key, depth as u8, score, TTFlag::LowerBound, None);
            return score;
        }
        if score > best_score {
            best_score = score;
        }
        if score > alpha {
            alpha = score
        }
    }

    let flag = if best_score <= alpha_orig {
        TTFlag::UpperBound
    } else if best_score >= beta {
        TTFlag::LowerBound
    } else {
        TTFlag::Exact
    };
    search_info.tt_stores += 1;
    tt.store(key, depth as u8, best_score, flag, None);

    best_score
}
