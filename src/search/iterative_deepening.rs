use num_format::Locale;
use num_format::ToFormattedString;

use crate::prelude::*;
use crate::search;
use crate::search::search_info::SearchInfo;
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
    let start_time = Instant::now();

    for depth in 1..=max_depth {
        let start = Instant::now();
        let mut search_info = SearchInfo {
            total_nodes: 0,
            stop_signal: stop.clone(),
            timeout_occurred: false,
        };

        let (current_depth_best_move, current_depth_best_eval) = search::alpha_beta::alpha_beta(
            board,
            depth,
            i32::MIN + 1,
            i32::MAX,
            &stop,
            &mut search_info,
        );

        // If the timeout occured in the last minimax than we need to check if it maybe found a better position even though it was canceled
        // However we need to check if it reached went to the desired depth, if not it could throw bad eval values
        if search_info.timeout_occurred {
            if current_depth_best_move.is_some() && current_depth_best_eval > best_eval_overall {
                best_move_overall = current_depth_best_move;
                best_eval_overall = current_depth_best_eval;
            }
        // Normal case if no timeout hapenned
        } else if current_depth_best_move.is_some() {
            best_move_overall = current_depth_best_move;
            best_eval_overall = current_depth_best_eval;
        }

        let best_move_string = match best_move_overall {
            Some(mv) => mv.decode().to_coords(),
            None => "0000".to_string(),
        };

        let elapsed = start.elapsed();
        let nodes_per_seconds = (search_info.total_nodes as f64 / elapsed.as_secs_f64()) as usize;
        let canceled_str = match search_info.timeout_occurred {
            true => "canceled",
            false => "",
        };

        println!(
            "info  depth={}  {}  {}cp  {}nps  {}nodes  {:.3}s  {}",
            depth.to_formatted_string(&Locale::en),
            best_move_string,
            best_eval_overall.to_formatted_string(&Locale::en),
            nodes_per_seconds.to_formatted_string(&Locale::en),
            search_info.total_nodes.to_formatted_string(&Locale::en),
            elapsed.as_secs_f64(),
            canceled_str,
        );

        if search_info.timeout_occurred {
            break;
        }
    }

    best_move_overall
}
