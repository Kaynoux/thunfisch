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

        // iter through search args
        let mut iter = args.iter();
        while let Some(&tok) = iter.next() {
            match tok {
                "wtime" => {
                    if let Some(&val) = iter.next() {
                        wtime = val.parse().unwrap_or(0)
                    }
                }
                "btime" => {
                    if let Some(&val) = iter.next() {
                        btime = val.parse().unwrap_or(0)
                    }
                }
                "winc" => {
                    if let Some(&val) = iter.next() {
                        winc = val.parse().unwrap_or(0)
                    }
                }
                "binc" => {
                    if let Some(&val) = iter.next() {
                        binc = val.parse().unwrap_or(0)
                    }
                }
                "movestogo" => {
                    if let Some(&val) = iter.next() {
                        movestogo = val.parse().unwrap_or(0)
                    }
                }
                "movetime" => {
                    if let Some(&val) = iter.next() {
                        movetime = val.parse().unwrap_or(0)
                    }
                }
                _ => {}
            }
        }

        // Calculate time budget
        let have_tc = wtime > 0 || btime > 0 || movetime > 0;
        let search_time = if !have_tc {
            // no time control at all
            Duration::new(24 * 3600, 0)
        } else if movetime > 0 {
            Duration::from_millis(movetime)
        } else {
            // get current color specific values
            let (time_left, inc) = if board.current_color() == White {
                (wtime, winc)
            } else {
                (btime, binc)
            };
            // if no movestogo set we estimate it is 30 (very dumb change later)
            let moves_to_go = if movestogo > 0 { movestogo } else { 30 };
            let mut time_per_move = time_left / moves_to_go;
            // add half of increment (safe that way than 100%)
            time_per_move += inc / 2;
            // safety margins
            let safety = std::cmp::max(time_left / 20, 50);
            // never take up all time
            if time_per_move + safety >= time_left {
                time_per_move = time_left.saturating_sub(safety);
            }
            // Minimum 10 ms
            if time_per_move < 10 {
                time_per_move = 10;
            }
            Duration::from_millis(time_per_move)
        };

        let best_move = iterative_deepening::iterative_deepening(board, 100, search_time);

        if let Some(mv) = best_move {
            println!("info pv {}", mv.decode().to_coords());
            println!("bestmove {}", mv.decode().to_coords());
        } else if board.is_in_check() {
            println!("Game over: Checkmate!");
        } else {
            println!("Game over: Stalemate!");
        }
    }
}
