use std::time::Duration;

use crate::debug::perft;
use crate::prelude::*;
use crate::search::iterative_deepening;
pub fn bestmove(args: Vec<&str>, board: &mut Board) {
    if args.len() >= 2 && args[0] == "perft" {
        let depth: usize = match args[1].parse() {
            Ok(d) => d,
            Err(_) => 0,
        };

        let debug = args.iter().any(|&flag| flag == "--debug");
        let perftree = args.iter().any(|&flag| flag == "--perftree");
        let rayon = args.iter().any(|&flag| flag == "--rayon");

        if debug == true {
            perft::perft_debug(board, depth);
        } else if rayon == true {
            perft::perft_rayon(board, depth);
        } else if perftree == true {
            perft::perft_perftree_format(board, depth);
        } else {
            perft::perft(board, depth);
        }
    } else if args.len() >= 2 && args[0] == "depth" {
        let depth: usize = match args[1].parse() {
            Ok(d) => d,
            Err(_) => 0,
        };

        let best_move = iterative_deepening::iterative_deepening(
            board,
            depth,
            Duration::new(24 * 3600, 0), // use 24h as upper limit
        );

        if let Some(best_move) = best_move {
            println!("bestmove {}", best_move.decode().to_coords());
        } else {
            if board.is_in_check() {
                println!("Game over: Checkmate!");
            } else {
                println!("Game over: Stalemate!");
            }
        }
    } else {
        let mut wtime: u64 = 0;
        let mut btime: u64 = 0;
        let mut winc: u64 = 0;
        let mut binc: u64 = 0;
        let mut movestogo: u64 = 0;
        let mut movetime: u64 = 0;

        let mut i = 0;
        while i < args.len() {
            match args[i] {
                "wtime" => {
                    if let Some(v) = args.get(i + 1) {
                        wtime = v.parse().unwrap_or(0);
                    }
                }
                "btime" => {
                    if let Some(v) = args.get(i + 1) {
                        btime = v.parse().unwrap_or(0);
                    }
                }
                "winc" => {
                    if let Some(v) = args.get(i + 1) {
                        winc = v.parse().unwrap_or(0);
                    }
                }
                "binc" => {
                    if let Some(v) = args.get(i + 1) {
                        binc = v.parse().unwrap_or(0);
                    }
                }
                "movestogo" => {
                    if let Some(v) = args.get(i + 1) {
                        movestogo = v.parse().unwrap_or(0);
                    }
                }
                "movetime" => {
                    if let Some(v) = args.get(i + 1) {
                        movetime = v.parse().unwrap_or(0);
                    }
                }
                _ => {}
            }
            i += 1;
        }

        // Calculate time budget
        let have_time_control = wtime > 0 || btime > 0 || movetime > 0;
        let search_time = if !have_time_control {
            // no time control given
            Duration::new(24 * 3600, 0) // use 24h as upper limit
        } else if movetime > 0 {
            Duration::from_millis(movetime)
        } else {
            // normal engine time control
            let (time_left, inc) = if board.current_color() == White {
                (wtime, winc)
            } else {
                (btime, binc)
            };
            let moves = if movestogo > 0 { movestogo } else { 40 };
            let mut base_ms = (time_left / moves) + inc;

            // little bit of safety
            if base_ms > 50 {
                base_ms -= 30;
            }
            Duration::from_millis(base_ms)
        };

        // Start search with calcualte search time
        let best_move = iterative_deepening::iterative_deepening(board, 100, search_time);

        if let Some(mv) = best_move {
            println!("bestmove {}", mv.decode().to_coords());
        } else if board.is_in_check() {
            println!("Game over: Checkmate!");
        } else {
            println!("Game over: Stalemate!");
        }
    }
}
