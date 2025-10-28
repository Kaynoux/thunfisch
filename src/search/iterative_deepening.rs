use super::transposition_table::TT;
use crate::prelude::*;
use crate::search::alpha_beta;
use crate::search::move_ordering;
use crate::settings::settings;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use std::time::Instant;
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

/// https://www.chessprogramming.org/Iterative_Deepening
pub fn iterative_deepening(
    board: &mut Board,
    max_depth: usize,
    time_limit: Duration,
) -> Option<EncodedMove> {
    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop_clone = stop.clone();
        thread::spawn(move || {
            thread::sleep(time_limit);
            stop_clone.store(true, Ordering::Relaxed);
        });
    }

    let mut best_move_overall: Option<EncodedMove> = None;
    let mut best_eval_overall = i32::MIN;
    let global_start = Instant::now();

    for depth in 1..=max_depth {
        let iteration_start = Instant::now();
        let iteration_search_info = SearchInfo::new();

        let mut root_moves = board.generate_moves::<false>();
        if settings::MOVE_ORDERING {
            move_ordering::order_moves(&mut root_moves, board);
        }

        // format: z(best move, evaluation after move is made, seldepth)
        let results: Vec<(EncodedMove, i32, usize)> = root_moves
            .par_iter()
            .map(|&mv| {
                let mut b = board.clone(); // Board muss Clone implementieren
                b.make_move(&mv.decode());
                let mut local_seldepth = 1;
                let eval = -alpha_beta::alpha_beta(
                    &mut b,
                    depth - 1,
                    -i32::MAX,
                    i32::MAX,
                    &stop,
                    &iteration_search_info,
                    1,
                    &mut local_seldepth,
                )
                .1;
                (mv, eval, local_seldepth)
            })
            .collect();

        let best_result_local = results
            .into_iter()
            .max_by_key(|&(_mv, eval, _seldepth)| eval);

        let (best_move_local, best_eval_local, best_seldepth) = match best_result_local {
            Some((mv, eval, seldepth)) => (Some(mv), eval, seldepth),
            None => (None, 0, 0),
        };

        if !iteration_search_info
            .timeout_occurred
            .load(Ordering::Relaxed)
        {
            best_move_overall = best_move_local;
            best_eval_overall = best_eval_local;
        }

        let mut mv = match best_move_overall {
            Some(mv) => mv.decode(),
            None => {
                return best_move_overall;
            }
        };

        // Lazy way to promote always to Queen because my evaluation is dumb sometimes
        // - if it thinks it looses the piece fast it tries to minimize the cost by selecting a worse piece to begin with
        let mv_type = mv.mv_type;
        mv.mv_type = match mv_type {
            MoveType::RookPromoCapture
            | MoveType::BishopPromoCapture
            | MoveType::KnightPromoCapture => MoveType::QueenPromoCapture,
            MoveType::RookPromo | MoveType::BishopPromo | MoveType::KnightPromo => {
                MoveType::QueenPromo
            }
            _ => mv_type,
        };

        // generate a string with the best moves after each oterh known as pv string
        // Starts by doing the current best move and adds it to the pv string this is then repeated by always doing the bestmove and then looking up the best move from the tt table again and again until the tt does not contain a bestmove for the board state at that moment
        let pv_string = if let Some(root_mv) = best_move_overall {
            let mut pv_moves = Vec::new();
            pv_moves.push(root_mv);

            let mut b = board.clone();
            b.make_move(&root_mv.decode());

            let mut cnt = 1;
            while cnt < depth {
                if let Some(mv) = TT.probe(b.hash()) {
                    b.make_move(&mv.decode());
                    pv_moves.push(mv);
                    // First do and add the move to pv string if then a threefold repition is triggered cancel further pv string generation
                    if b.is_threefold_repetition() || b.is_50_move_rule() {
                        break;
                    }
                } else {
                    break;
                }
                cnt += 1;
            }

            // convert to coords
            pv_moves
                .iter()
                .map(|m| m.decode().to_coords())
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            String::new()
        };

        let iteration_ab_nodes = iteration_search_info
            .total_alpha_beta_nodes
            .load(Ordering::Relaxed);
        let iteration_qs_nodes = iteration_search_info.total_qs_nodes.load(Ordering::Relaxed);
        let iteration_eval_nodes = iteration_search_info
            .total_eval_nodes
            .load(Ordering::Relaxed);

        let iteration_not_eval_nodes = iteration_ab_nodes + iteration_qs_nodes;
        let iteration_nodes = iteration_not_eval_nodes + iteration_eval_nodes;

        let iteration_duration = iteration_start.elapsed();
        let global_duration = global_start.elapsed();
        let nodes_per_seconds =
            (iteration_nodes as f64 / iteration_duration.as_secs_f64()) as usize;

        let current_color_multiplier = match board.current_color() {
            White => 1,
            Black => -1,
        };

        println!(
            "info  depth {} seldepth {}  score cp {} nodes {} nps {} time {} tt {} pv {} | nodes_ab {} nodes_qs {} timeout {} total_time {}",
            depth,
            best_seldepth,
            best_eval_overall * current_color_multiplier,
            iteration_nodes,
            nodes_per_seconds,
            iteration_duration.as_millis(),
            TT.fill_ratio().2 as usize,
            pv_string,
            iteration_ab_nodes,
            iteration_qs_nodes,
            iteration_search_info
                .timeout_occurred
                .load(Ordering::Relaxed),
            global_duration.as_millis()
        );

        if iteration_search_info
            .timeout_occurred
            .load(Ordering::Relaxed)
        {
            break;
        }
    }

    best_move_overall
}
