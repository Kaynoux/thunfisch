use crate::debug;
use crate::prelude::*;
use crate::search;
use std::io::{self, BufRead, Write};
use std::time::Duration;

pub fn handle_uci_communication() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut state = EngineState::new(); // State enthält jetzt das Board

    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => break,
        };

        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("uci") => {
                println!("id name MeinRustBot");
                println!("id author DeinName");
                println!("uciok");
            }
            Some("isready") => {
                println!("readyok");
            }
            Some("ucinewgame") => {
                state = EngineState::new();
            }
            Some("position") => {
                let args: Vec<&str> = parts.collect();
                state.handle_position(&args);
            }
            Some("go") => {
                let args: Vec<&str> = parts.collect();

                if args.len() == 2 && args[0] == "depth" {
                    let depth: usize = match args[1].parse() {
                        Ok(d) => d,
                        Err(_) => 0,
                    };

                    let best_move = search::iterative_deepening::iterative_deepening(
                        &mut state.board,
                        depth,
                        Duration::new(3600, 0),
                    );

                    if let Some(best_move) = best_move {
                        println!("bestmove {}", best_move.decode().to_coords());
                    } else {
                        println!("bestmove (none)");
                    }

                    continue;
                }

                let best_move = search::iterative_deepening::iterative_deepening(
                    &mut state.board,
                    5,
                    Duration::new(1, 0),
                );

                if let Some(best_move) = best_move {
                    println!("bestmove {}", best_move.decode().to_coords());
                } else {
                    if state.board.is_in_check() {
                        println!("Game over: Checkmate!");
                    } else {
                        println!("Game over: Stalemate!");
                    }
                }
            }
            Some("quit") => break,
            Some("perft") => {
                let args: Vec<&str> = parts.collect();
                let depth: usize = match args[0].parse() {
                    Ok(d) => d,
                    Err(_) => 0,
                };

                let debug = args.iter().any(|&flag| flag == "--debug");
                let rayon = args.iter().any(|&flag| flag == "--rayon");

                if debug == true {
                    debug::debug_perft(&mut state.board, depth);
                } else if rayon == true {
                    debug::perft_rayon(&mut state.board, depth);
                } else {
                    debug::perft(&mut state.board, depth);
                }
            }
            Some("fen") => println!("Current Fen: {}", state.board.generate_fen()),
            Some("draw") => debug::print_board(&state.board, None),
            Some("moves") => {
                let moves = state.board.get_moves(false);
                debug::print_board(&state.board, Some(&moves));
            }
            Some("eval") => loop {},
            Some("do") => {
                let args: Vec<&str> = parts.collect();
                let mv_str: &str = args[0];
                let mv = DecodedMove::from_coords(mv_str.to_string(), &state.board);
                state.board.make_move(&mv);
            }
            Some(cmd) => {
                eprintln!("Unknown command: {}", cmd);
            }
            None => {}
        }

        stdout.flush().unwrap();
    }
}
