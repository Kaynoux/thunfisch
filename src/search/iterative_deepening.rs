use super::transposition_table::TT;
use crate::debug::visualize::format_f64;
use crate::debug::visualize::format_usize;
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
    debug: bool,
    help: bool,
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
    let mut previouse_iteration_ab_nodes: usize = 0;
    let mut previouse_iteration_qs_nodes: usize = 0;

    if debug {
        if help {
            println!("Depth   : Current iterative deepening depth (plies)");
            println!("Seldepth: Maximum depth reached due to QS extensions");
            println!("Score   : Position evaluation from engine's perspective");
            println!("Nodes   : Total number of nodes searched (AB + QS)");
            println!("NPS     : Nodes per second processed in this iteration");
            println!("LocTime : Time taken for the current iteration (ms)");
            println!("TT%     : Percentage of Transposition Table filled");
            println!("AB Nodes: Nodes visited in standard Alpha-Beta");
            println!("QS nodes: Nodes visited in Quiescence search");
            println!("TT Hits : Times a TT entry was reused");
            println!("GlobTime: Total elapsed time since search started (ms)");
            println!("EBF     : Effective Branch Factor (relative to the previous depth)");
            println!("AB EBF  : Alpha Beta Nodes Only Effective Branch Factor");
            println!("PV      : Sequence of moves that programs consider best");
            println!()
        }
        println!(
            "Activated Features: AB={:?} QS={:?} TT-AB={:?} TT-QS={:?} MVV-LVA={:?} QS-Check-Evasion-Limit={:?} Order-TT-Moves-First={:?}",
            settings::AB,
            settings::QS,
            settings::TT_AB,
            settings::TT_QS,
            settings::MVV_LVA,
            settings::QS_CHECK_EVASION_LIMIT,
            settings::ORDER_TT_MV_FIRST,
        );
        println!(
            "TT: Age={}   {} of {} Entries {} % full  Allocated Size: {}B",
            TT.get_age(),
            format_usize(TT.info().0),
            format_usize(TT.info().1),
            format_f64(TT.info().2,),
            format_usize(TT.info().3)
        );
        println!();

        if !help {
            println!("Use go --debug --help to get every column explained");
        }

        println!(
            "{:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {}",
            "Depth",
            "Seldepth",
            "Score",
            "Nodes",
            "NPS",
            "LocTime",
            "TT%",
            "AB Nodes",
            "QS Nodes",
            "TT Hits",
            "GlobTime",
            "EBF",
            "AB EBF",
            "PV"
        );
    }

    for depth in 1..=max_depth {
        let iteration_start = Instant::now();
        let iteration_search_info = SearchInfo::new();

        let mut root_moves = board.generate_moves::<false>();

        let mut tt_move: Option<EncodedMove> = None;
        if settings::TT_AB {
            // only probe TT after draw detection, because the TT does not remember move history
            // doing it the other way around yield the TT score instead of the draw score for a repetition
            if let Some(tt_hit) = TT.probe(board.hash(), 0) {
                iteration_search_info
                    .total_tt_hits
                    .fetch_add(1, Ordering::Relaxed);

                tt_move = tt_hit.best_move();
            }
        }

        move_ordering::order_moves(&mut root_moves, board, tt_move);

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
        let iteration_not_eval_nodes = iteration_ab_nodes + iteration_qs_nodes;
        let iteration_nodes = iteration_not_eval_nodes + iteration_eval_nodes;

        let iteration_duration = iteration_start.elapsed();
        let nodes_per_seconds =
            (iteration_nodes as f64 / iteration_duration.as_secs_f64()) as usize;

        match debug {
            false => {
                println!(
                    "info  depth {} seldepth {}  score cp {} nodes {} nps {} time {} tt {} pv {}",
                    depth,
                    best_seldepth,
                    best_eval_overall,
                    iteration_nodes,
                    nodes_per_seconds,
                    iteration_duration.as_millis(),
                    TT.info().2 as usize,
                    pv,
                );
            }
            true => {
                let iteration_tt_hits = iteration_search_info.total_tt_hits.load(Ordering::Relaxed);
                let global_duration = global_start.elapsed();

                let ebf = if (previouse_iteration_ab_nodes + previouse_iteration_qs_nodes) > 0 {
                    iteration_nodes as f64
                        / (previouse_iteration_ab_nodes + previouse_iteration_qs_nodes) as f64
                } else {
                    0.0
                };

                let ab_ebf = if previouse_iteration_ab_nodes > 0 {
                    iteration_ab_nodes as f64 / previouse_iteration_ab_nodes as f64
                } else {
                    0.0
                };

                println!(
                    "{:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {}",
                    depth,
                    best_seldepth,
                    best_eval_overall,
                    format_usize(iteration_nodes),
                    format_usize(nodes_per_seconds),
                    format_usize(iteration_duration.as_millis() as usize),
                    format_f64(TT.info().2),
                    format_usize(iteration_ab_nodes),
                    format_usize(iteration_qs_nodes),
                    format_usize(iteration_tt_hits),
                    format_usize(global_duration.as_millis() as usize),
                    format_f64(ebf),
                    format_f64(ab_ebf),
                    pv
                );
            }
        }

        previouse_iteration_ab_nodes = iteration_ab_nodes;
        previouse_iteration_qs_nodes = iteration_qs_nodes;

        if iteration_search_info
            .timeout_occurred
            .load(Ordering::Relaxed)
        {
            break;
        }
    }

    best_pv.last().cloned()
}
