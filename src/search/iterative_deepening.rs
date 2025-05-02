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
) -> Option<ChessMove> {
    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop_clone = stop.clone();
        thread::spawn(move || {
            thread::sleep(time_limit);
            stop_clone.store(true, Ordering::Relaxed);
        });
    }

    let mut best_move_overall: Option<ChessMove> = None;
    let mut best_eval_overall = i32::MIN;
    let mut total_nodes_overall = 0usize;
    let start_time = Instant::now();

    for depth in 1..=max_depth {
        let mut search_info = SearchInfo {
            total_parent_nodes: 0,
            total_nodes: 0,
            stop_signal: stop.clone(),
            timeout_occurred: false,
        };

        let (best_current_move, best_current_eval) = search::alpha_beta::alpha_beta(
            board,
            depth,
            i32::MIN + 1,
            i32::MAX,
            &stop,
            &mut search_info,
        );

        total_nodes_overall += search_info.total_nodes;

        // If the timeout occured in the last minimax than we need to check if it maybe found a better position even though it was canceled
        // However we need to check if it reached went to the desired depth, if not it could throw bad eval values
        if best_current_move.is_some() {
            best_move_overall = best_current_move;
            best_eval_overall = best_current_eval;
        }

        let best_move_string = match best_move_overall {
            Some(mv) => mv.to_coords(),
            None => "0000".to_string(),
        };

        println!(
            "info depth: {} score: {}cp bestmove: {} nodes: {} canceled: {}",
            depth.to_formatted_string(&Locale::en),
            best_eval_overall.to_formatted_string(&Locale::en),
            best_move_string,
            total_nodes_overall.to_formatted_string(&Locale::en),
            search_info.timeout_occurred
        );

        if search_info.timeout_occurred {
            break;
        }
    }

    best_move_overall
}
