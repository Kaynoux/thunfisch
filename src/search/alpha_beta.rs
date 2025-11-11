use super::move_ordering;
use super::transposition_table::{ScoreType, TT};
use crate::prelude::*;
use crate::search::quiescence_search;
use crate::settings::settings;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

const MATE_SCORE: i32 = 100000;
const MAX_QUIESCENCE_SEARCH_DEPTH: usize = 12;

/// https://www.chessprogramming.org/Alpha-Beta
/// Returns the pv, and the associated evaluation
/// Note that the pv is reversed, i.e. the best move at this depth is at the end of the list
pub fn alpha_beta(
    board: &mut Board,
    depth: usize,
    mut alpha: i32,
    beta: i32,
    stop: &Arc<AtomicBool>,
    search_info: &SearchInfo,
    ply: usize,
    local_seldepth: &mut usize,
) -> (Vec<EncodedMove>, i32) {
    *local_seldepth = (*local_seldepth).max(ply);
    search_info
        .total_alpha_beta_nodes
        .fetch_add(1, Ordering::Relaxed);

    if board.is_threefold_repetition() || board.is_50_move_rule() {
        return (vec![], 0);
    }

    let hash = board.hash();

    // only probe TT after draw detection, because the TT does not remember move history
    // doing it the other way around yield the TT score instead of the draw score for a repetition
    if let Some(tt_hit) = TT.probe(hash, alpha, beta, depth) {
        search_info.total_tt_hits.fetch_add(1, Ordering::Relaxed);
        return (vec![tt_hit.1], tt_hit.0);
    }

    if depth == 0 {
        if settings::QUIESCENCE_SEARCH {
            let qs_result = quiescence_search::quiescence_search(
                board,
                alpha,
                beta,
                MAX_QUIESCENCE_SEARCH_DEPTH,
                stop,
                search_info,
                ply,
                local_seldepth,
            );

            TT.store(hash, None, qs_result, depth, ScoreType::Exact);
            return (vec![], qs_result);
        }
        return (vec![], board.evaluate());
    };

    // cancels search if time is over
    if stop.load(Ordering::Relaxed) {
        search_info.timeout_occurred.store(true, Ordering::Relaxed);
        return (vec![], 0);
    }

    // set the best evaluation very low to begin with
    let mut best_eval = i32::MIN + 1;
    let mut best_move: Option<EncodedMove> = None;

    let mut moves = board.generate_moves::<false>();

    // returns the mate score (very low) when in check but subtracts the depth to give a later check a better eval because the depth is lowers the further you go
    if moves.is_empty() {
        if board.is_in_check() {
            return (vec![], -MATE_SCORE - (depth as i32));
        } else {
            return (vec![], 0);
        }
    }

    if settings::MOVE_ORDERING {
        move_ordering::order_moves(&mut moves, board);
    }

    // Probe again as there's a chance a different thread updated the TT since we last probed it
    // Also probe with maximum possible window to ensure we get an entry if it exists
    if let Some((_, tt_mv)) = TT.probe(hash, i32::MAX, i32::MIN, 0) {
        if let Some(pos) = moves.iter().position(|&m| m == tt_mv) {
            moves.swap(0, pos);
        }
    }

    let mut score_type = ScoreType::UpperBound;
    let mut pv = vec![];
    for mv in moves {
        // cancels search if time is over
        if stop.load(Ordering::Relaxed) {
            search_info.timeout_occurred.store(true, Ordering::Relaxed);
            return (vec![], 0);
        }
        board.make_move(&mv.decode());
        let (mut local_pv, mut eval) = alpha_beta(
            board,
            depth - 1,
            -beta,
            -alpha,
            stop,
            search_info,
            ply + 1,
            local_seldepth,
        );
        // do the negamax negation here since we can't negate a tuple above
        eval *= -1;
        board.unmake_move();

        if eval > best_eval {
            best_eval = eval;
            best_move = Some(mv);
            local_pv.extend(vec![mv]);
            pv = local_pv;
            if eval > alpha {
                alpha = eval;
                score_type = ScoreType::Exact;
            }
        }

        if settings::ALPHA_BETA {
            if eval >= beta {
                // beta cutoff -> there could be better moves we didn't see, so the score is lower bound
                TT.store(hash, best_move, eval, depth, ScoreType::LowerBound);
                return (pv, best_eval);
            }
        }
    }

    TT.store(hash, best_move, best_eval, depth, score_type);

    (pv, best_eval)
}
