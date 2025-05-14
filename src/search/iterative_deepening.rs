use super::transposition_table::TT;
use crate::prelude::*;
use crate::search::alpha_beta;
use num_format::Locale;
use num_format::ToFormattedString;
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

    for depth in 1..=max_depth {
        let start = Instant::now();
        let stop_signal = Arc::new(AtomicBool::new(false));
        let search_info = SearchInfo::new(stop_signal);

        let root_moves = board.generate_moves::<false>();

        let results: Vec<(EncodedMove, i32)> = root_moves
            .par_iter()
            .map(|&mv| {
                let mut b = board.clone(); // Board muss Clone implementieren
                b.make_move(&mv.decode());
                let eval = -alpha_beta::alpha_beta(
                    &mut b,
                    depth - 1,
                    -i32::MAX,
                    i32::MAX,
                    &stop,
                    &search_info, // ggf. separate SearchInfo pro Thread
                )
                .1;
                (mv, eval)
            })
            .collect();

        let best_result_local = results.into_iter().max_by_key(|&(_mv, eval)| eval);
        let (best_move_local, best_eval_local) = match best_result_local {
            Some((mv, eval)) => (Some(mv), eval),
            None => (None, 0),
        };

        if !search_info.timeout_occurred.load(Ordering::Relaxed) {
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

        let total_ab_nodes = search_info.total_alpha_beta_nodes.load(Ordering::Relaxed);
        let total_qs_nodes = search_info.total_qs_nodes.load(Ordering::Relaxed);
        let total_eval = search_info.total_eval_nodes.load(Ordering::Relaxed);

        let total_not_eval_nodes = total_ab_nodes + total_qs_nodes;
        let total_nodes = total_not_eval_nodes + total_eval;

        let elapsed = start.elapsed();
        let nodes_per_seconds = (total_nodes as f64 / elapsed.as_secs_f64()) as usize;

        let canceled = search_info.timeout_occurred.load(Ordering::Relaxed);
        let canceled_str = if canceled { "canceled" } else { "" };

        println!(
            "info  depth={}  best={}  eval={}cp  nps={}  nodes={}  {:.3}s  as={:.3}  qs={:.3} tt={:.1}% {}/{} {}",
            depth.to_formatted_string(&Locale::en),
            mv.to_coords(),
            best_eval_overall.to_formatted_string(&Locale::en),
            nodes_per_seconds.to_formatted_string(&Locale::en),
            total_nodes.to_formatted_string(&Locale::en),
            elapsed.as_secs_f64(),
            total_ab_nodes as f64 / (total_not_eval_nodes as f64),
            total_qs_nodes as f64 / (total_not_eval_nodes as f64),
            TT.fill_ratio().2,
            TT.fill_ratio().0.to_formatted_string(&Locale::en),
            TT.fill_ratio().1.to_formatted_string(&Locale::en),
            canceled_str,
        );

        if search_info.timeout_occurred.load(Ordering::Relaxed) {
            break;
        }
    }

    best_move_overall
}
