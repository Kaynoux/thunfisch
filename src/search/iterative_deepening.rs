use super::transposition_table::TT;
use crate::prelude::*;
use crate::search::alpha_beta;
use crate::search::alpha_beta::alpha_beta;
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

    for depth in 1..=max_depth {
        let start = Instant::now();
        let search_info = SearchInfo::new();

        let root_moves = board.generate_moves::<false>();

        // format: z(best move, evaluation after move is made, seldepth)
        // let results: Vec<(EncodedMove, i32, usize)> = root_moves
        //     .par_iter()
        //     .map(|&mv| {
        //         let mut b = board.clone(); // Board muss Clone implementieren
        //         b.make_move(&mv.decode());
        //         let mut local_seldepth = 1;
        //         let eval = -alpha_beta::alpha_beta(
        //             &mut b,
        //             depth - 1,
        //             -i32::MAX,
        //             i32::MAX,
        //             &stop,
        //             &search_info,
        //             1,
        //             &mut local_seldepth,
        //         )
        //         .1;
        //         (mv, eval, local_seldepth)
        //     })
        //     .collect();

        let mut local_seldepth = 0;
        let best_result_local = alpha_beta::alpha_beta(
            board,
            depth,
            -i32::MAX,
            i32::MAX,
            &stop,
            &search_info,
            0,
            &mut local_seldepth,
        );
        let best_result_local = Some((
            best_result_local.0.unwrap(),
            -best_result_local.1, // invert for negamax
            local_seldepth,
        ));

        let (best_move_local, best_eval_local, best_seldepth) = match best_result_local {
            Some((mv, eval, seldepth)) => (Some(mv), eval, seldepth),
            None => (None, 0, 0),
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

        // generate principal variation string (i.e. the best moves for both sides)
        let pv_string = if let Some(root_mv) = best_move_overall {
            let mut pv_moves = Vec::new();
            pv_moves.push(root_mv);

            let mut b = board.clone();
            b.make_move(&root_mv.decode());

            let mut cnt = 1;
            while cnt < depth {
                if let Some(mv) = TT.probe(b.hash()) {
                    pv_moves.push(mv);
                    b.make_move(&mv.decode());
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

        let total_ab_nodes = search_info.total_alpha_beta_nodes.load(Ordering::Relaxed);
        let total_qs_nodes = search_info.total_qs_nodes.load(Ordering::Relaxed);
        let total_eval = search_info.total_eval_nodes.load(Ordering::Relaxed);

        let total_not_eval_nodes = total_ab_nodes + total_qs_nodes;
        let total_nodes = total_not_eval_nodes + total_eval;

        let elapsed = start.elapsed();
        let nodes_per_seconds = (total_nodes as f64 / elapsed.as_secs_f64()) as usize;
        // chess evaluations are always absolute (+ = advantage white, - = advantage black)
        // negamax however always puts advantages relative to the player
        let current_color_multiplier = match board.current_color() {
            White => 1,
            Black => -1,
        };

        println!(
            "info  depth {} seldepth {}  score cp {} nodes {} nps {} time {} tt {} pv {} | nodes_ab {} nodes_qs {}",
            depth,
            best_seldepth,
            best_eval_overall * current_color_multiplier,
            total_nodes,
            nodes_per_seconds,
            elapsed.as_millis(),
            TT.fill_ratio().2 as usize,
            pv_string,
            total_ab_nodes,
            total_qs_nodes,
        );

        if search_info.timeout_occurred.load(Ordering::Relaxed) {
            break;
        }
    }

    best_move_overall
}
