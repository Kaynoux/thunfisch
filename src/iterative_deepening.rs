use crate::{
    alpha_beta::alpha_beta,
    debug::visualize::{format_f64, format_usize},
    move_scoring::HISTORY_TABLE,
    prelude::*,
    transposition_table::TT,
};

use crate::{settings, settings::MAX_AB_DEPTH};

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

/// <https://www.chessprogramming.org/Iterative_Deepening>
#[allow(clippy::too_many_lines)]
pub fn iterative_deepening(
    board: &mut Board,
    max_depth: usize,
    time_limit: Duration,
    debug: bool,
    help: bool,
) -> Option<EncodedMove> {
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
            println!("LMR Res : Total Late Move Reduction re-searches");
            println!("PVS Res : Total Principal Variation Search re-searches");
            println!("GlobTime: Total elapsed time since search started (ms)");
            println!(
                "EBF     : Effective Branch Factor (Relative to the previous depth iteration)"
            );
            println!("AB EBF  : Alpha Beta Nodes Only Effective Branch Factor");
            println!("PV      : Sequence of moves that programs consider best");
            println!();
        }
        println!("{}", settings::repr());
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
            "{:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} PV",
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
            "LMR Res",
            "PVS Res",
            "GlobTime",
            "EBF",
            "AB EBF"
        );
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////////

    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop_clone = stop.clone();
        thread::spawn(move || {
            thread::sleep(time_limit);
            stop_clone.store(true, Ordering::Relaxed);
        });
    }

    let mut best_eval_overall;
    let mut best_pv: Vec<EncodedMove> = Vec::new();
    let global_start = Instant::now();
    let mut previouse_iteration_ab_nodes: usize = 0;
    let mut previouse_iteration_qs_nodes: usize = 0;
    let mut killers = [EncodedMove(0); MAX_AB_DEPTH];

    HISTORY_TABLE.age();
    for depth in 1..=max_depth {
        let iteration_start = Instant::now();
        let mut seldepth = 0;
        let mut iteration_search_data =
            SharedSearchData::new(board, &stop, &mut seldepth, &mut killers);

        let best_eval_local = alpha_beta::<true>(
            depth,
            -i32::MAX,
            i32::MAX,
            &mut iteration_search_data,
            0,
            false,
        );

        if iteration_search_data
            .timeout_occurred
            .load(Ordering::Relaxed)
        {
            break;
        }

        best_eval_overall = best_eval_local;
        let mut pv_local: Vec<EncodedMove> = Vec::new();

        // TT-Walk to obtain the PV
        let mut b = iteration_search_data.board.clone();
        let mut ply: i32 = 0;
        while pv_local.len() < depth
            && let Some(tt_entry) = TT.probe(b.hash(), ply)
            && let Some(tt_mv) = tt_entry.best_move()
            && !b.is_50_move_rule()
            && !b.is_threefold_repetition()
        {
            pv_local.push(tt_mv);
            b.make_move(tt_mv);
            ply += 1;
        }

        best_pv = pv_local;

        let pv_string = best_pv
            .iter()
            .map(|emv| emv.decode().to_coords())
            .collect::<Vec<_>>()
            .join(" ");

        let iteration_ab_nodes = iteration_search_data
            .total_alpha_beta_nodes
            .load(Ordering::Relaxed);
        let iteration_qs_nodes = iteration_search_data.total_qs_nodes.load(Ordering::Relaxed);
        let iteration_eval_nodes = iteration_search_data
            .total_eval_nodes
            .load(Ordering::Relaxed);
        let iteration_not_eval_nodes = iteration_ab_nodes + iteration_qs_nodes;
        let iteration_nodes = iteration_not_eval_nodes + iteration_eval_nodes;

        let iteration_duration = iteration_start.elapsed();

        let nodes_per_seconds = if iteration_duration.is_zero() {
            0
        } else {
            (iteration_nodes.saturating_mul(1000))
                / (iteration_duration.as_millis() as usize).max(1)
        };

        #[allow(clippy::cast_precision_loss)]
        if debug {
            let iteration_tt_hits = iteration_search_data.total_tt_hits.load(Ordering::Relaxed);
            let iteration_lmr_researches = iteration_search_data
                .total_lmr_researches
                .load(Ordering::Relaxed);
            let iteration_pvs_researches = iteration_search_data
                .total_pvs_researches
                .load(Ordering::Relaxed);
            let global_duration = global_start.elapsed();

            let current_total_nodes = iteration_nodes as f64;
            let previous_total_nodes =
                (previouse_iteration_ab_nodes + previouse_iteration_qs_nodes) as f64;

            let ebf = if previous_total_nodes > 0.0 {
                current_total_nodes / previous_total_nodes
            } else {
                0.0
            };

            let ab_ebf = if previouse_iteration_ab_nodes > 0 {
                iteration_ab_nodes as f64 / previouse_iteration_ab_nodes as f64
            } else {
                0.0
            };

            println!(
                "{:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {}",
                depth,
                seldepth,
                best_eval_overall,
                format_usize(iteration_nodes),
                format_usize(nodes_per_seconds),
                format_usize(iteration_duration.as_millis() as usize),
                format_f64(TT.info().2),
                format_usize(iteration_ab_nodes),
                format_usize(iteration_qs_nodes),
                format_usize(iteration_tt_hits),
                format_usize(iteration_lmr_researches),
                format_usize(iteration_pvs_researches),
                format_usize(global_duration.as_millis() as usize),
                format_f64(ebf),
                format_f64(ab_ebf),
                pv_string
            );
        } else {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let fill_rate = TT.info().2 as usize;
            println!(
                "info  depth {} seldepth {}  score cp {} nodes {} nps {} time {} tt {} pv {}",
                depth,
                seldepth,
                best_eval_overall,
                iteration_nodes,
                nodes_per_seconds,
                iteration_duration.as_millis(),
                fill_rate,
                pv_string,
            );
        }

        previouse_iteration_ab_nodes = iteration_ab_nodes;
        previouse_iteration_qs_nodes = iteration_qs_nodes;
    }

    best_pv.first().copied()
}
