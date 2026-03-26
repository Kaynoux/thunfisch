use super::transposition_table::TT;
use crate::prelude::*;
use crate::search::alpha_beta;
use crate::search::move_ordering;
use crate::settings::settings;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use std::i32;
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

    let mut best_pv: Vec<EncodedMove> = vec![];
    let mut best_eval_overall = i32::MIN;
    let global_start = Instant::now();

    for depth in 1..=max_depth {
        let iteration_start = Instant::now();
        let iteration_search_info = SearchInfo::new();

        let mut root_moves = board.generate_moves::<false>();
        if settings::MOVE_ORDERING {
            move_ordering::order_moves(&mut root_moves, board);
        }

        // format: (pv, evaluation after move is made, seldepth)
        let results: Vec<(Vec<EncodedMove>, i32, usize)> = root_moves
            .par_iter()
            .map(|&mv| {
                let mut b = board.clone(); // Board muss Clone implementieren
                b.make_move(&mv.decode());
                let mut local_seldepth = 1;
                let (mut pv, mut eval) = alpha_beta::alpha_beta(
                    &mut b,
                    depth - 1,
                    -i32::MAX,
                    i32::MAX,
                    &stop,
                    &iteration_search_info,
                    1,
                    &mut local_seldepth,
                    false
                );
                eval *= -1;
                pv.extend(vec![mv]);
                (pv, eval, local_seldepth)
            })
            .collect();

        let best_result_local = results.iter().max_by_key(|&(_mv, eval, _seldepth)| eval);

        let (best_pv_local, best_eval_local, best_seldepth) = match best_result_local {
            Some((mv, eval, seldepth)) => (mv.clone(), *eval, *seldepth),
            None => (vec![], 0, 0),
        };

        if !iteration_search_info
            .timeout_occurred
            .load(Ordering::Relaxed)
        {
            best_pv = best_pv_local;
            best_eval_overall = best_eval_local;
        }

        let pv = best_pv
            .iter()
            .rev()
            .map(|emv| emv.decode().to_coords())
            .collect::<Vec<_>>()
            .join(" ");

        let iteration_ab_nodes = iteration_search_info
            .total_alpha_beta_nodes
            .load(Ordering::Relaxed);
        let iteration_qs_nodes = iteration_search_info.total_qs_nodes.load(Ordering::Relaxed);
        let iteration_eval_nodes = iteration_search_info
            .total_eval_nodes
            .load(Ordering::Relaxed);
        let iteration_tt_hits = iteration_search_info.total_tt_hits.load(Ordering::Relaxed);

        let iteration_not_eval_nodes = iteration_ab_nodes + iteration_qs_nodes;
        let iteration_nodes = iteration_not_eval_nodes + iteration_eval_nodes;

        let iteration_duration = iteration_start.elapsed();
        let global_duration = global_start.elapsed();
        let nodes_per_seconds =
            (iteration_nodes as f64 / iteration_duration.as_secs_f64()) as usize;

        println!(
            "info  depth {} seldepth {}  score cp {} nodes {} nps {} time {} tt {} pv {} | nodes_ab {} nodes_qs {} tt_hits {} timeout {} total_time {}",
            depth,
            best_seldepth,
            best_eval_overall,
            iteration_nodes,
            nodes_per_seconds,
            iteration_duration.as_millis(),
            TT.fill_ratio().2 as usize,
            pv,
            iteration_ab_nodes,
            iteration_qs_nodes,
            iteration_tt_hits,
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

    best_pv.last().cloned()
}
